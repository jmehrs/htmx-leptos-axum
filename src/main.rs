use std::sync::{Arc, Mutex};

use anyhow::Result;
use axum::{
    response::Html,
    routing::get,
    Router
};
use htmx_leptos_axum::{
    init_db,
    devices::{device_router, DeviceAdd}
};
use leptos::{ssr::render_to_string as render, *};
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use tower_http::services::ServeDir;


async fn index() -> Html<String> {
    Html(render(|cx| {
        view! { cx,
            <head>
                <link href="./assets/output.css" rel="stylesheet" />
                <script type="text/javascript" src="./assets/htmx.min.js" />
                <script type="text/javascript" src="./assets/hyperscript.min.js" />
            </head>
            <body>
                <nav>"NAVBAR"</nav>

                <main>
                    <DeviceAdd />
                </main>

            </body>
        }
    }))
}


#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let conn = init_db()?;
    let conn = Arc::new(Mutex::new(conn));

    let app = Router::new()
        .route("/", get(index))
        .nest("/device", device_router())
        .with_state(conn)
        .nest_service("/assets", ServeDir::new("assets"));
        
    info!("Starting server at http://0.0.0.0:8080");
    axum::Server::bind(
        &"0.0.0.0:8080"
            .parse()
            .expect("Failed to bind to 0.0.0.0:8080"),
    )
        .serve(app.into_make_service())
        .await
        .expect("Error serving application");
    Ok(())
}
