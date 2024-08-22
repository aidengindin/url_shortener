use rocket::fs::NamedFile;
use rocket::form::Form;

#[macro_use] extern crate rocket;

#[derive(FromForm)]
struct Url<'r> {
    r#link: &'r str,
}

#[get("/")]
async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("index.html").await
}

#[post("/new", data = "<url>")]
fn new(url: Form<Url<'_>>) {
    println!("got a post request from a form");
    // generate a random shortened url
    // redirect the user to a success page
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, new])
}
