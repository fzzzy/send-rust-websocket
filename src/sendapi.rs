
use std::io::prelude::*;
use std::fs::File;

use actix_web::{http, server, App, Path, HttpRequest, HttpResponse, Result};

const HTML: &'static str = include_str!("websockets.html");

fn home(_req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(HTML))
}

fn download(id: Path<String>) -> Result<HttpResponse> {
    let mut file = File::open(format!("{}.send", id)).unwrap();
    let mut contents = vec![];
    println!("Download {}", id);
    file.read_to_end(&mut contents).unwrap();
    // response
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("application/octet-stream")
        .body(contents))
}

pub fn api() {
    server::new(
        || App::new()
            .resource("/", |r| r.method(http::Method::GET).f(home))
            .route("/download/{id}/", http::Method::GET, download))
        .bind("127.0.0.1:12224").unwrap()
        .run();
}
