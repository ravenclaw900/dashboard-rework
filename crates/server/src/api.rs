use config::CONFIG;
use hyper::{header, StatusCode};
use hyper_ext::{
    upgrade_websocket, ErrorResponse, FullResponse, IncomingReq, IntoResponse, RequestExt,
    ResponseExt, UriExt, WsMessage,
};
use pty_process::Size;
use serde::Deserialize;
use sysdata::{Request, RequestTx};
use tracing::instrument;

pub async fn login(req: IncomingReq) -> FullResponse {
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

pub fn terminal(req: IncomingReq) -> FullResponse {
    use futures_util::{SinkExt, StreamExt};
    use pty_process::{Command, Pty};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    upgrade_websocket(req, |mut socket| async move {
        let mut pty = Pty::new().unwrap();
        let mut cmd = Command::new("bash");
        let mut child = cmd.spawn(&pty.pts().unwrap()).unwrap();

        let mut data = [0_u8; 256];

        loop {
            tokio::select! {
                Ok(num_read) = pty.read(&mut data) => {
                    if socket.send(WsMessage::Binary(data[..num_read].to_vec())).await.is_err() {
                        break;
                    }
                }
                res = socket.next() => {
                    let Some(Ok(msg)) = res else {
                        break;
                    };
                    match msg {
                        WsMessage::Text(size) if size.starts_with("size") => {
                            let size = size.trim_start_matches("size");
                            let Some((cols, rows)) = size
                                .split_once(',')
                                .and_then(|(cols, rows)| Some((cols.parse().ok()?, rows.parse().ok()?)))
                            else {
                                continue;
                            };
                            pty.resize(Size::new(rows, cols)).unwrap();
                        }
                        WsMessage::Binary(_) | WsMessage::Text(_) => {
                            pty.write_all(&msg.into_data()).await.unwrap();
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
