use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, Rng};
use rocket::fs::NamedFile;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::response::content::RawHtml;

#[macro_use] extern crate rocket;

#[derive(FromForm)]
struct Url<'r> {
    r#link: &'r str,
}

static URL_MAP: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[get("/")]
async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("index.html").await
}

#[post("/new", data = "<url>")]
fn new(url: Form<Url<'_>>) -> Redirect {

    // generate a random shortened url
    let short_url: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    let mut map = URL_MAP.lock().unwrap();
    map.insert(short_url.clone(), url.link.to_string());

    // redirect the user to a success page
    Redirect::to(format!("/success/{}", short_url))
}

#[get("/success/<short_url>")]
fn success(short_url: &str) -> RawHtml<String> {
    RawHtml(format!("
        <p>
            Your shortened URL is: <a href=\"http://localhost:8000/{}\">http://localhost:8000/{}</a>
        </p>
    ", short_url, short_url))
}

#[get("/<short_url>")]
fn redirect(short_url: &str) -> Redirect {
    let map = URL_MAP.lock().unwrap();
    match map.get(short_url) {
        Some(url) => Redirect::to(url.clone()),
        None => Redirect::to("/not-saved")
    }
}

// TODO: 404
#[get("/not-saved")]
fn not_saved() -> &'static str {
    "That URL hasn't been saved."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, new, success, redirect, not_saved])
}
