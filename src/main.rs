use std::{net::SocketAddr, time::Duration};

use axum::{extract::State, routing::get, Json, Router};
use ephemeropt::EphemeralOption;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::{mpsc, oneshot};
use types::cpu::Cpu;
use vite_embed::ViteEmbed;
use vite_embed_axum::{vite_embed, vite_router};

mod types;

#[cfg(not(feature = "dev"))]
const FRONTEND: ViteEmbed = vite_embed_axum::vite_embed::generate_vite_prod!(
    "$CARGO_MANIFEST_DIR/frontend/dist/manifest.json"
);

#[cfg(feature = "dev")]
const FRONTEND: ViteEmbed = vite_embed_axum::vite_embed::generate_vite_dev!(
    "$CARGO_MANIFEST_DIR/frontend/index.html",
    "src/index.tsx"
);

fn round_percent(float: f32) -> f32 {
    (float * 100.).round() / 100.
}

async fn cpu_api(State(sys_tx): State<mpsc::Sender<SystemRequest>>) -> Json<Cpu> {
    let (resp_tx, resp_rx) = oneshot::channel();
    sys_tx.send(SystemRequest::Cpu(resp_tx)).await.unwrap();
    let val = resp_rx.await.unwrap();
    Json(Cpu { usage: val })
}

enum SystemRequest {
    Cpu(oneshot::Sender<f32>),
}

fn system_task() -> mpsc::Sender<SystemRequest> {
    let mut sys = System::new();
    let mut cpu_opt = EphemeralOption::new_empty(Duration::from_millis(900));
    let (tx, mut rx) = mpsc::channel(10);
    tokio::task::spawn(async move {
        while let Some(request) = rx.recv().await {
            match request {
                SystemRequest::Cpu(channel) => match cpu_opt.get() {
                    Some(&cpu) => channel.send(cpu).unwrap(),
                    None => {
                        sys.refresh_cpu();
                        let cpu = round_percent(sys.global_cpu_info().cpu_usage());
                        cpu_opt.insert(cpu);
                        channel.send(cpu).unwrap();
                    }
                },
            };
        }
    });
    tx
}

#[tokio::main]
async fn main() {
    let asset_router = vite_router(FRONTEND);
    let sys_tx = system_task();

    let router = Router::new()
        .route("/api/cpu-usage", get(cpu_api))
        .with_state(sys_tx)
        .merge(asset_router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
