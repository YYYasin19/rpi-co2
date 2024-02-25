use axum::{routing::get, Json, Router};
use rand::Rng;
use serde::Serialize;
use std::{fs::File, io::Read};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// use tracing_subscriber;
#[derive(Serialize)]
struct Co2Data {
    timestamps: Vec<String>,
    co2values: Vec<i32>,
}

async fn data() -> Json<Co2Data> {
    let mut file = File::open("values.csv").expect("Unable to open values.csv");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let mut rdr = csv::Reader::from_reader(contents.as_bytes());
    let mut co2values: Vec<i32> = Vec::new();
    let mut timestamps: Vec<String> = Vec::new();
    for result in rdr.records() {
        let record = result.expect("Unable to read record");
        timestamps.push(record[0].to_string());
        let ppm: i32 = record[1].parse().expect("Unable to parse ppm");
        co2values.push(ppm);
    }

    Json(Co2Data {
        timestamps,
        co2values,
    })
}

async fn dummy_data() -> Json<Co2Data> {
    let mut rng = rand::thread_rng();
    let co2values: Vec<i32> = (0..100).map(|_| rng.gen_range(10..100)).collect();
    let timestamps: Vec<String> = (0..100)
        .map(|i| {
            let now = chrono::Utc::now();
            let dt = now - chrono::Duration::seconds(100 - i);
            dt.to_rfc3339()
        })
        .collect();
    // print before returning

    Json(Co2Data {
        timestamps,
        co2values,
    })
}

async fn hello() -> &'static str {
    "Hello world!!"
}

async fn echo(req_body: String) -> String {
    req_body
}

#[tokio::main]
async fn main() {
    // init tracing subscriber so we see logs in the terminal
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(hello))
        .route("/echo", get(echo))
        .route("/data", get(data))
        .route("/dummy_data", get(dummy_data))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .nest_service("/ui", ServeDir::new("./rpi-co2-ui/dist/"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
