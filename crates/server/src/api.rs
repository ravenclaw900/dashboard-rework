use config::CONFIG;
use hyper::{header, StatusCode};
use hyper_ext::{
    upgrade_websocket, ErrorResponse, HttpResponse, IncomingReq, IntoResponse, RequestExt,
    ResponseExt, UriExt, WsMessage,
};
use pty_process::Size;
use serde::Deserialize;
use sysdata::{Request, RequestTx};
use tracing::instrument;

pub async fn login(req: IncomingReq) -> HttpResponse {
    let body = req.into_body_bytes().await;
    let mut resp = "".into_response();

    let Some(pass) = body.strip_prefix(b"pass=") else {
        *resp.status_mut() = StatusCode::BAD_REQUEST;
        return resp;
    };

    if !auth::test_password(pass) {
        resp.redirect("/login?incorrect=");
        return resp;
    }

    let token = auth::create_token();

    let cookie_header = format!(
        "token={}; SameSite=Lax; Max-Age={}; HttpOnly; Path=/",
        token, CONFIG.auth.expiry
    );

    resp.insert_header(header::SET_COOKIE, &cookie_header);
    resp.redirect("/");

    resp
}

#[derive(Deserialize)]
pub struct ProcessSignalQuery {
    pid: usize,
    signal: sysdata::types::ProcessSignal,
}

#[instrument(skip_all, err)]
pub async fn process_signal(req: IncomingReq, tx: RequestTx) -> Result<(), ErrorResponse> {
    let query: ProcessSignalQuery = req
        .uri()
        .deserialize_query()
        .map_err(|_| ErrorResponse::new_client_err(ErrorResponse::QUERY_MSG))?;

    tx.send(Request::ProcessSignal(query.pid, query.signal))
        .await
        .map_err(|_| ErrorResponse::new_server_err("Failed to send message"))?;

    Ok(())
}

pub fn terminal(req: IncomingReq) -> Result<HttpResponse, ErrorResponse> {
    use futures_util::{SinkExt, StreamExt};
    use pty_process::{Command, Pty};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    // Hoist some of the work out of the closure, so an error can be easily returned
    let mut pty = Pty::new().map_err(|_| ErrorResponse::new_server_err("failed to spawn pty"))?;
    let pts = pty
        .pts()
        .map_err(|_| ErrorResponse::new_server_err("failed to get pts from pty"))?;
    let mut cmd = Command::new("bash");
    let mut child = cmd
        .spawn(&pts)
        .map_err(|_| ErrorResponse::new_server_err("failed to spawn child process"))?;

    upgrade_websocket(req, |mut socket| async move {
        let mut data = [0_u8; 256];

        loop {
            tokio::select! {
                Ok(num_read) = pty.read(&mut data) => {
                    let send_result = socket.send(WsMessage::Binary(data[..num_read].to_vec())).await;
                    // If sending fails, socket was likely closed on the other end
                    if send_result.is_err() {
                        break;
                    }
                }
                res = socket.next() => {
                    let Some(Ok(msg)) = res else {
                        break;
                    };
                    match msg {
                        // I don't like doing it like this (always a chance that someone manages to input their own size message)
                        // but I'm not sure there's a much more elegant way
                        WsMessage::Text(size) if size.starts_with("size") => {
                            let size = size.trim_start_matches("size");
                            let Some((cols, rows)) = size
                                .split_once(',')
                                .and_then(|(cols, rows)| Some((cols.parse().ok()?, rows.parse().ok()?)))
                            else {
                                continue;
                            };
                            // Ignore resize result, likely doesn't necessarily mean pty is closed
                            let _ = pty.resize(Size::new(rows, cols));
                        }
                        WsMessage::Binary(_) | WsMessage::Text(_) => {
                            let write_res = pty.write_all(&msg.into_data()).await;
                            // If writing to pty fails, pty is closed and terminal should close
                            if write_res.is_err() {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        let _ = child.kill().await;
        let _ = child.wait().await;
    })
}
