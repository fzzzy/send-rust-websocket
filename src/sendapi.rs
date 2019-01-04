
use actix_web::{http, server, App, Path, Responder, HttpRequest, HttpResponse, Result};

const HTML: &'static str = include_str!("websockets.html");

fn home(_req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(HTML))
}

fn download(id: Path<String>) -> impl Responder {
    let out = format!("Hello {}!\n", id);
    println!("{}", out);
    out
}

pub fn api() {
    server::new(
        || App::new()
            .resource("/", |r| r.method(http::Method::GET).f(home))
            .route("/download/{id}", http::Method::GET, download))
        .bind("127.0.0.1:8088").unwrap()
        .run();
}
