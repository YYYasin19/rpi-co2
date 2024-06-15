use axum::{response::Html, routing::get, Router};
use clap::Parser;
use rand::Rng;
use std::env;
mod sensor;
use minijinja::{context, Environment};
use sensor::Sensor;
use serde::Serialize;
use std::{fs::File, io::Read};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// use tracing_subscriber;
#[derive(Serialize)]
struct Co2Data {
    timestamps: Vec<String>,
    co2values: Vec<i32>,
}

async fn data() -> Co2Data {
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

    Co2Data {
        timestamps,
        co2values,
    }
}

async fn health() -> &'static str {
    "ok"
}

async fn dummy_data() -> Co2Data {
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

    Co2Data {
        timestamps,
        co2values,
    }
}

/// Dummy endpoint
async fn echo(req_body: String) -> String {
    req_body
}

/// Render a template using minijinja
async fn render_template(data: Co2Data) -> Html<String> {
    // TODO: instantiate this once instead of everytime
    let mut env = Environment::new();
    env.add_template("index.jinja2", include_str!("templates/index.jinja2"))
        .unwrap();

    let template = env.get_template("index.jinja2").unwrap();
    let rendered = template.render(context! { data => data }).unwrap();
    Html(rendered)
}

/// reads the data and returns a rendered template as HTML response
async fn index() -> Html<String> {
    let data = data().await;
    render_template(data).await
}

/// reads dummy data and returns a rendered template as HTML response
async fn dummy_index() -> Html<String> {
    let data = dummy_data().await;
    render_template(data).await
}

/// shows debug and configuration information
async fn show_config() -> String {
    let co2_device = env::var("CO2_DEVICE").unwrap_or("/dev/ttyAMA0".to_string());
    format!("CO2_DEVICE: {}", co2_device)
}

fn run_sensor(device: String, mock_mode: bool) {
    if mock_mode {
        println!("Running in mock mode");
        let mut sensor = Sensor::new_mock(device).unwrap();
        sensor.read_ppm_loop_mock();
    } else {
        println!("Reading from device: /dev/ttyAMA0");
        let mut sensor = Sensor::new(device).unwrap();
        sensor.read_ppm_loop();
    }
}

#[derive(Parser)]
struct ServerCli {
    #[clap(long, short, action)]
    mock: bool,
    #[clap(long, default_value = "./rpi-co2-ui/dist/")]
    ui_path: String,
    #[clap(long, default_value = "3000")]
    server_port: u16,
}

#[tokio::main]
async fn main() {
    // init tracing subscriber so we see logs in the terminal
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // accept option --ui with path to custom folder or default to rpi-co2-ui/dist
    let cli_args = ServerCli::parse();

    println!("Starting server with UI path: {}", cli_args.ui_path);
    let app = Router::new()
        .route("/", get(index))
        .route("/dummy", get(dummy_index))
        .route("/health", get(health))
        .route("/echo", get(echo))
        .route("/config", get(show_config))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());
    // .nest_service("/ui", ServeDir::new(cli_args.ui_path.clone()));

    // start a new thread for the sensor
    let co2_device = env::var("CO2_DEVICE").unwrap_or("/dev/ttyAMA0".to_string());
    let mock_mode = cli_args.mock;
    std::thread::spawn(move || {
        run_sensor(co2_device, mock_mode.clone());
    });

    println!("Listening on port {}", cli_args.server_port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{:?}", cli_args.server_port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
