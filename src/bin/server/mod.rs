use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::{fs::File, io::Read};

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

async fn hello() -> &'static str {
    "Hello world!!"
}

async fn echo(req_body: String) -> String {
    req_body
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/echo", get(echo))
        .route("/data", get(data));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
