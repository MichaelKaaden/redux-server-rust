use actix_web::{get, web, App, HttpServer, Responder};

struct AppInfo {
    version: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppInfo {
                version: { format!("v{}", built_info::PKG_VERSION) },
            })
            .service(version)
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/counters")
                    // ...so this handles requests for "GET /app/index.html"
                    .route("/index.html", web::get().to(index)),
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

async fn index() -> impl Responder {
    "Hello world"
}

#[get("/version")]
async fn version(data: web::Data<AppInfo>) -> impl Responder {
    format!("{}", &data.version)
}

// Use of a mod or pub mod is not actually necessary.
pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
