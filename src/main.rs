use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

use redux_server_rust::Counter;

struct AppInfo {
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    counters: Mutex<HashMap<u32, i32>>, // Mutex is necessary to mutate safely across threads
}

#[derive(Debug, Serialize, Deserialize)]
struct CounterDTO {
    counter: Counter,
}

#[derive(Debug, Serialize, Deserialize)]
struct CountersDTO {
    counters: Vec<Counter>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JSONResult<T> {
    data: T,
    message: String,
    status: u32,
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
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/demo")
                    // ...so this handles requests for "GET /app/index.html"
                    .route("/index.html", web::get().to(index)),
            )
            .service(get_counter)
            .service(get_counters)
            .service(get_version)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

async fn index() -> impl Responder {
    "Hello world"
}

#[get("/version")]
async fn get_version(data: web::Data<AppInfo>) -> impl Responder {
    data.version.to_string()
}

#[get("/counters")]
async fn get_counters(data: web::Data<AppState>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(JSONResult {
        data: CountersDTO {
            counters: redux_server_rust::get_counters(data.counters.lock().unwrap()),
        },
        message: "okay".to_string(),
        status: 200,
    }))
}

#[get("/counters/{counter_id}")]
async fn get_counter(
    data: web::Data<AppState>,
    web::Path(counter_id): web::Path<u32>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(JSONResult {
        data: CounterDTO {
            counter: redux_server_rust::get_counter(data.counters.lock().unwrap(), counter_id),
        },
        message: "okay".to_string(),
        status: 200,
    }))
}

// Use of a mod or pub mod is not actually necessary.
pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
