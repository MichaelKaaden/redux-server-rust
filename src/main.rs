use std::collections::HashMap;
use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{get, http, put, web, App, HttpResponse, HttpServer, Responder, Result};
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
struct CounterDto {
    counter: Counter,
}

#[derive(Debug, Serialize, Deserialize)]
struct CountersDto {
    counters: Vec<Counter>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonResult<T> {
    data: T,
    message: String,
    status: u32,
}

#[derive(Deserialize)]
struct JsonBodySet {
    count: i32,
}

#[derive(Deserialize)]
struct JsonBodyChangeBy {
    by: i32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut initial_counters = HashMap::new();
    initial_counters.insert(0, 0); // define an initial counter
    let app_state = web::Data::new(AppState {
        counters: Mutex::new(initial_counters),
    });

    HttpServer::new(move || {
        // limit the payload size to 1_024 bytes and log errors
        let json_config = web::JsonConfig::default()
            .limit(1024)
            .error_handler(|err, req| {
                println!("err: {:?}", err);
                println!("req: {:#?}", req);
                // produce your own error, here: Conflict
                //error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                //    .into()
                err.into()
            });

        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_methods(vec!["GET", "HEADER", "OPTIONS", "PUT"])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            // handle CORS
            .wrap(cors)
            // shared app state
            .app_data(app_state.clone())
            .app_data(json_config)
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
            .service(decrement)
            .service(increment)
            .service(set_counter)
    })
    //.bind("127.0.0.1:3000")?
    .bind("0.0.0.0:3000")?
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
    Ok(HttpResponse::Ok().json(JsonResult {
        data: CountersDto {
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
    Ok(HttpResponse::Ok().json(JsonResult {
        data: CounterDto {
            counter: redux_server_rust::get_counter(data.counters.lock().unwrap(), counter_id),
        },
        message: "okay".to_string(),
        status: 200,
    }))
}

#[put("/counters/{counter_id}")]
async fn set_counter(
    data: web::Data<AppState>,
    web::Path(counter_id): web::Path<u32>,
    value: web::Json<JsonBodySet>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(JsonResult {
        data: CounterDto {
            counter: redux_server_rust::set_counter(
                data.counters.lock().unwrap(),
                counter_id,
                value.count,
            ),
        },
        message: "okay".to_string(),
        status: 200,
    }))
}

#[put("/counters/{counter_id}/decrement")]
async fn decrement(
    data: web::Data<AppState>,
    web::Path(counter_id): web::Path<u32>,
    value: web::Json<JsonBodyChangeBy>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(JsonResult {
        data: CounterDto {
            counter: redux_server_rust::decrement_counter(
                data.counters.lock().unwrap(),
                counter_id,
                value.by,
            ),
        },
        message: "okay".to_string(),
        status: 200,
    }))
}

#[put("/counters/{counter_id}/increment")]
async fn increment(
    data: web::Data<AppState>,
    web::Path(counter_id): web::Path<u32>,
    value: web::Json<JsonBodyChangeBy>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(JsonResult {
        data: CounterDto {
            counter: redux_server_rust::increment_counter(
                data.counters.lock().unwrap(),
                counter_id,
                value.by,
            ),
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
