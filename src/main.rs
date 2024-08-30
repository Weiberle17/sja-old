use axum::Router;
use leptos_axum::LeptosRoutes;
use tower_http::compression::CompressionLayer;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Warn).expect("Couldn't initialize logging");

    let conf = leptos::get_configuration(None)
        .await
        .expect("Error getting Leptos config");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = leptos_axum::generate_route_list(sja::app::App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, sja::app::App)
        .with_state(leptos_options)
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Error setting up TcpListener");
    log::info!("listening on http://{}", &addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
