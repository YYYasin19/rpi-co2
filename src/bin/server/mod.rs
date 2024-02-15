#[macro_use]
extern crate rocket;
use std::vec;

use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    // read the values.csv file
    let mut rdr = csv::Reader::from_path("values.csv").expect("Unable to open values.csv");
    let mut co2values: Vec<i32> = vec![];
    let mut timestamps: Vec<String> = vec![];
    for result in rdr.records() {
        let record = result.expect("Unable to read record");
        timestamps.push(record[0].to_string());
        let ppm: i32 = record[1].parse().expect("Unable to parse ppm");
        co2values.push(ppm);
    }

    Template::render(
        "index",
        context! {
            name: "Ferris",
            cool: "true",
            timestamps: timestamps,
            co2values: co2values,
        },
    )
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
        .attach(Template::fairing())
}
