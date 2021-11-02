#[macro_use]
extern crate rocket;

use rocket::data::{Data, ToByteUnit};
use std::io;
use std::path::Path;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<file>")]
async fn upload(file: Data<'_>) -> io::Result<String> {
    file.open(128.gigabytes())
        .into_file(Path::new("./uploads/test"))
        .await?;
    Ok(format!("test"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload])
}
