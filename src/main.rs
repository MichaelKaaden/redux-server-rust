use std::sync::Mutex;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

struct AppInfo {
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    counter: Mutex<Counter>, // Mutex is necessary to mutate safely across threads
}

#[derive(Debug, Serialize, Deserialize)]
struct Counter {
    index: i32,
    value: i32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(Counter { index: 0, value: 0 }),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .data(AppInfo {
                version: { format!("v{}", built_info::PKG_VERSION) },
            })
            .service(version)
            .service(counters)
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/demo")
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

#[get("/counters")]
async fn counters(data: web::Data<AppState>) -> Result<HttpResponse> {
    let counter = data.counter.lock().unwrap();
    Ok(HttpResponse::Ok().json(Counter {
        index: counter.index,
        value: counter.value,
    }))
}

// Use of a mod or pub mod is not actually necessary.
pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
