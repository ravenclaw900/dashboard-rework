use super::api;
use crate::sysdata;
use axum::{routing::get, Router};

#[cfg(feature = "frontend")]
use vite_embed_axum::vite_embed::{self, ViteEmbed};
#[cfg(feature = "frontend")]
use vite_embed_axum::vite_router;

#[cfg(all(not(feature = "dev"), feature = "frontend"))]
const FRONTEND: ViteEmbed = vite_embed_axum::vite_embed::generate_vite_prod!(
    "$CARGO_MANIFEST_DIR/frontend/dist/manifest.json"
);

// If dev is enabled frontend has to be enabled
#[cfg(feature = "dev")]
const FRONTEND: ViteEmbed = vite_embed_axum::vite_embed::generate_vite_dev!(
    "$CARGO_MANIFEST_DIR/frontend/index.html",
    "src/index.tsx"
);

fn api_router() -> Router {
    let tx = sysdata::spawn_system_task();
    Router::new()
        .route("/system", get(api::system))
        .with_state(tx)
}

pub fn router() -> Router {
    let router = Router::new();

    #[cfg(feature = "frontend")]
    let router = router.merge(vite_router(FRONTEND));

    router.nest("/api", api_router())
}
