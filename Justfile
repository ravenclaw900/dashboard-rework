alias release := backend-release

frontend:
    #!/bin/bash -eux
    cd frontend/
    pnpm install
    pnpm build

backend: frontend
    rm -f ./target/debug/deps/dietpi_dashboard-*
    cargo build

backend-release: frontend
    cargo build --release --features frontend

backend-only:
    cargo build --release

ci target backend-only:
    #!/bin/bash -eux
    if {{backend-only}}; then
        cross build --target {{target}} --release --no-default-features
    else
        just frontend
        cross build --target {{target}} --release
    fi

dev:
    #!/bin/bash -eux
    cd frontend/
    pnpm install
    pnpm dev &
    cargo run --features dev &
    trap 'kill $(jobs -pr)' EXIT
    wait