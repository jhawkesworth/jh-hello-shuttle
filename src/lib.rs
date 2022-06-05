#[macro_use]
extern crate rocket;



// use rocket_dyn_templates::{Template, context};
use rocket::{Build, Rocket};
use rocket::response::status;
use rocket::request::FromParam;

struct IntTemp {
    degrees: f32
}

impl<'a> FromParam<'a> for IntTemp<> {
    type Error = &'a str;
    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param.chars().all(|c| c.is_alphanumeric() || c.eq_ignore_ascii_case(&'-') || c.eq_ignore_ascii_case(&'.'))
            .then(|| IntTemp{degrees: param.parse::<f32>().unwrap()})
            .ok_or(param)
    }
    // TODO can't be lower than -459.67 F / -273 C
}

#[catch(404)] fn not_found() -> &'static str { "Nothing here, sorry!" }
#[catch(500)] fn just_500() -> &'static str { "Whoops!?" }
// #[catch(default)] fn some_default() -> &'static str { "Everything else." }

// #[get("/hello")]
// fn hello() -> Template {
//     Template::render("hello", context! { greeting: "Hello, world!  I am a rocket shuttle app", })
// }

// #[get("/")]
// fn index() -> Template {
//     Template::render("index", context! { greeting: "Hello, world!  I am an index page", })
// }

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world! I am an index page"
}

#[get("/ctof/<celsius>")]
fn c_to_f(celsius: IntTemp) -> Option<String> {
    let converted = (celsius.degrees * 9.0 / 5.0 ) +32.0;
    Option::from(converted.to_string())
}

#[get("/ftoc/<farenheit>")]
fn f_to_c(farenheit: IntTemp) -> Option<String> {
    let converted = (farenheit.degrees - 32.0) * 5.0 / 9.0;
    Option::from(converted.to_string())
}



#[shuttle_service::main]
async fn rocket() -> Result<Rocket<Build>, shuttle_service::Error> {
    let rocket = rocket::build()
        .register("/duff", catchers![just_500])
        .register("/", catchers![not_found, just_500])
        .mount("/", routes![index, hello, c_to_f, f_to_c])
        // as of 0.3.0 shuttle just does not support anything external to the compiled .so
        // so we can't use rocket's dynamic template support at the moment.
        //.mount("/templates", routes![hello])
        //.attach(Template::fairing())
        ;

    Ok(rocket)
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn c_to_f_works() {
        // 10 C = 50 F.
        let result = c_to_f(IntTemp { degrees: 10 });
        assert_eq!(result, Option::Some(String::from("50")));
    }

    #[test]
    fn f_to_c_works() {
        // 50 F = 10 C.  ish
        let result = f_to_c(IntTemp { degrees: 50 });
        assert_eq!(result, Option::Some(String::from("10")));
    }

    #[test]
    fn dotallowed() {
        let c = '.';
        assert!(c.eq_ignore_ascii_case(&'.'));
    }
}