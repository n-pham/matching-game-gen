use dioxus::prelude::*;
use my_dioxus_app::routes::Route;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use my_dioxus_app::server::init_db;
    use dioxus_server::ServeConfig;
    use dioxus_server::server::DioxusRouterExt;

    // Initialize in-memory database
    init_db().await;

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server starting on http://{}", addr);

    let router = Router::new()
        .serve_dioxus_application(ServeConfig::new(), move || rsx! { Router::<Route> {} });

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::Config;
        let config = Config::new();
        dioxus::LaunchBuilder::new().with_cfg(config).launch(|| rsx! { Router::<Route> {} });
    }
    #[cfg(not(feature = "desktop"))]
    dioxus::launch(|| rsx! { Router::<Route> {} });
}
