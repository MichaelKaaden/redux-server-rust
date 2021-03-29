use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

struct AppInfo {
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    counters: Mutex<HashMap<u32, i32>>, // Mutex is necessary to mutate safely across threads
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut initial_counters = HashMap::new();
    initial_counters.insert(0, 0); // define an initial counter
    let app_state = web::Data::new(AppState {
        counters: Mutex::new(initial_counters),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .data(AppInfo {
                version: { format!("v{}", built_info::PKG_VERSION) },
            })
            .service(version)
            .service(get_counters)
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
async fn get_counters(data: web::Data<AppState>) -> Result<HttpResponse> {
    let counters = redux_server_rust::get_counters(data.counters.lock().unwrap());
    Ok(HttpResponse::Ok().json(counters))
}

// Use of a mod or pub mod is not actually necessary.
pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
