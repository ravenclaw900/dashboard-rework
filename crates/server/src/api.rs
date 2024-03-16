use axum::{
    extract::{ws, Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Redirect, Response},
};
use config::CONFIG;
use serde::Deserialize;
use sysdata::{Request, RequestTx};

pub async fn login(body: String) -> Response {
    let Some(pass) = body.strip_prefix("pass=") else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    if !auth::test_password(pass) {
        return Redirect::to("/login?incorrect=").into_response();
    }

    let token = auth::create_token();

    let cookie_header = format!(
        "token={}; SameSite=Lax; Max-Age={}; HttpOnly; Path=/",
        token, CONFIG.auth.expiry
    );

    ([(header::SET_COOKIE, cookie_header)], Redirect::to("/")).into_response()
}

#[derive(Deserialize)]
pub struct ProcessSignalQuery {
    signal: sysdata::types::ProcessSignal,
}

pub async fn process_signal(
    State(tx): State<RequestTx>,
    Path(pid): Path<usize>,
    signal: Query<ProcessSignalQuery>,
) {
    tx.send(Request::ProcessSignal(pid, signal.0.signal))
        .await
        .unwrap();
}

pub async fn terminal(ws: ws::WebSocketUpgrade) -> Response {
    use pty_process::{Command, Pty};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    ws.on_upgrade(|mut socket| async move {
        let mut pty = Pty::new().unwrap();
        let mut cmd = Command::new("bash");
        let mut child = cmd.spawn(&pty.pts().unwrap()).unwrap();

        let mut data = [0_u8; 256];

        loop {
            tokio::select! {
                Ok(num_read) = pty.read(&mut data) => {
                    if socket.send(ws::Message::Binary(data[..num_read].to_vec())).await.is_err() {
                        break;
                    }
                }
                res = socket.recv() => {
                    match res {
                        Some(Ok(msg)) => if matches!(msg, ws::Message::Binary(_) | ws::Message::Text(_)) {
                            pty.write_all(&msg.into_data()).await.unwrap();
                        },
                        _ => break,
                    }
                }
            }
        }

        child.kill().await.unwrap();
        child.wait().await.unwrap();
    })
}
