// © Copyright 2024 Jan Ehrhardt
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use cozyauth_server::app;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cozyauth_multi_tenant=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = app::app();
    let ip_address: IpAddr = if cfg!(debug_assertions) {
        Ipv6Addr::LOCALHOST.into()
    } else {
        Ipv6Addr::UNSPECIFIED.into()
    };
    let socket_address = SocketAddr::new(ip_address, app::server_port());
    let listener = TcpListener::bind(&socket_address).await.unwrap();
    info!("Listening on {}", socket_address);
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(app::shutdown_signal())
        .await
        .unwrap()
}
