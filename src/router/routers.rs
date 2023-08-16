use super::api;
use crate::sysdata;
use axum::{routing::get, Router};
use vite_embed_axum::vite_embed::{self, ViteEmbed};
use vite_embed_axum::vite_router;

#[cfg(not(feature = "dev"))]
const FRONTEND: ViteEmbed = vite_embed_axum::vite_embed::generate_vite_prod!(
    "$CARGO_MANIFEST_DIR/frontend/dist/manifest.json"
);

#[cfg(feature = "dev")]
const FRONTEND: ViteEmbed = vite_embed_axum::vite_embed::generate_vite_dev!(
    "$CARGO_MANIFEST_DIR/frontend/index.html",
    "src/index.tsx"
);

fn api_router() -> Router {
    let tx = sysdata::spawn_system_task();
    Router::new().route("/cpu", get(api::system)).with_state(tx)
}

pub fn router() -> Router {
    let asset_router = vite_router(FRONTEND);

    Router::new().nest("/api", api_router()).merge(asset_router)
}
