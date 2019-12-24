use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};


fn greet(req: HttpRequest) -> HttpResponse {
    let name = req.match_info().get("name").unwrap_or("World");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("Hello {}!", &name))
}

fn main() {
    // load ssl keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();

    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
    )
        .bind_ssl("127.0.0.1:8088", builder)
        .unwrap()
        .run()
        .unwrap();
}
