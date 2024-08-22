use rocket::fs::NamedFile;

#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("index.html").await
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
