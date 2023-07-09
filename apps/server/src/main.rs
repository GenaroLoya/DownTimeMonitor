#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    println!("HHHHHHHHH");
    "<h1>1234567789</h1>"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}

