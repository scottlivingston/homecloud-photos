#[macro_use]
extern crate rocket;
extern crate infer;

mod photo_upload;

#[launch]
fn rocket() -> _ {
    // rocket::build().mount("/", routes![index, photo_upload::photo_upload, file_upload])
    rocket::build().attach(photo_upload::stage())
}
