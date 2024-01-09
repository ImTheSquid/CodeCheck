use db::UserId;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use ::web::app::*;
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    let state = web::Data::new(::web::server::WebState::new().await.unwrap());

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;
        let state = state.clone();

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .service(logout)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(state)
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(feature = "ssr")]
#[actix_web::get("/logout")]
async fn logout(
    req: actix_web::HttpRequest,
    data: actix_web::web::Data<web::server::WebState>,
) -> impl actix_web::Responder {
    use actix_web::cookie::time::OffsetDateTime;
    use actix_web::cookie::Cookie;
    use actix_web::http::header::HeaderValue;
    match req.cookie("session") {
        None => {}
        Some(cookie) => {
            auth::logout(&data.database, cookie.value()).await.unwrap();
        }
    }

    // Clear session cookie if it exists
    let cookie = Cookie::build("session", "")
        .expires(Some(OffsetDateTime::now_utc()))
        .path("/")
        .finish();
    // response.insert_header(actix_web::http::header::SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).expect("Failed to write cookie"));

    let mut res = actix_web::HttpResponse::SeeOther().finish();
    res.headers_mut().insert(
        actix_web::http::header::LOCATION,
        "/?logout".parse().unwrap(),
    );
    res.headers_mut().insert(
        actix_web::http::header::SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).expect("Failed to write cookie"),
    );
    res
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use web::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
