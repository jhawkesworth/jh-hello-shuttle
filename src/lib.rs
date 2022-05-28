#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{Template, context};
use rocket::{Build, Rocket};

#[catch(404)] fn not_found() -> &'static str { "Nothing here, sorry!" }
// #[catch(500)] fn just_500() -> &'static str { "Whoops!?" }
// #[catch(default)] fn some_default() -> &'static str { "Everything else." }

#[get("/hello")]
fn hello() -> Template {
    Template::render("hello", context! { greeting: "Hello, world!  I am a rocket shuttle app", })
}
//
// #[get("/hello")]
// fn hello() -> &'static str {
//     "Hello, world!"
// }


#[get("/")]
fn index() -> &'static str {
    "Hello, world! I am an index page"
}

// #[get("/")]
// fn index() -> Template {
//     Template::render("index", context! { greeting: "Hello, world!  I am an index page", })
// }

#[shuttle_service::main]
async fn rocket() -> Result<Rocket<Build>, shuttle_service::Error> {
    let rocket = rocket::build()
        // .register("/duff", catchers![just_500, some_default])
        //.register("/foo", catchers![not_found])
        .mount("/", routes![index])
        .mount("/api", routes![hello])
        .attach(Template::fairing())
        ;

    Ok(rocket)
}