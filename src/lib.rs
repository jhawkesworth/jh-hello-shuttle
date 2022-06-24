#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
//use rocket::response::status;
use rocket::request::FromParam;
use rocket::response::content::{RawHtml, RawJavaScript};
use rocket::http::ContentType;

#[catch(404)]
fn not_found() -> &'static str {
    "Nothing here, sorry!"
}
#[catch(500)]
fn just_500() -> &'static str {
    "Whoops!?"
}

#[derive(Debug)]
enum Scale {
    Kelvin,
    Celsius,
    Farenheit,
}

impl Scale {
    fn minimum(&self) -> f32 {
        match *self {
            Scale::Kelvin => 0.0,
            Scale::Celsius => -273.15,
            Scale::Farenheit => -459.66,
        }
    }

    fn below_minimum(&self, value: &f32) -> bool {
        value < &self.minimum()
    }
}

impl<'a> FromParam<'a> for Scale {
    type Error = &'a str;
    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        if param.eq_ignore_ascii_case("celsius") || param.eq_ignore_ascii_case("c"){
            Ok(Scale::Celsius)
        } else if param.eq_ignore_ascii_case("farenheit") || param.eq_ignore_ascii_case("f"){
            Ok(Scale::Farenheit)
        } else if param.eq_ignore_ascii_case("kelvin") || param.eq_ignore_ascii_case("k"){
            Ok(Scale::Kelvin)
        } else {
            Err("could not understand scale needs to be exactly one of celsius/farenheit,kelvin")
        }
    }
}

fn ftoc(from: f32) -> f32 {
    (from - 32.0) * 5.0 / 9.0
}

fn ctof(from: f32) -> f32 {
    (from * 9.0 / 5.0) + 32.0
}

fn ktoc(from: f32) -> f32 {
    from - 273.15
}

fn ctok(from: f32) -> f32 {
    from + 273.15
}

fn convert(from: Scale, to: Scale, input: f32) -> f32 {
    match from {
        Scale::Celsius => match to {
            Scale::Farenheit => ctof(input),
            Scale::Kelvin => ctok(input),
            Scale::Celsius => input,
        },
        Scale::Farenheit => match to {
            Scale::Farenheit => input,
            Scale::Kelvin => ctok(ftoc(input)),
            Scale::Celsius => ftoc(input),
        },
        Scale::Kelvin => match to {
            Scale::Farenheit => ktoc(ctof(input)),
            Scale::Kelvin => input,
            Scale::Celsius => ktoc(input),
        },
    }
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../static_index.html"))
}

// instructions on how to build the game and compile into wasm
// are here: https://hands-on-rust.com/2021/11/06/run-your-rust-games-in-a-browser-hands-on-rust-bonus-content/
#[get("/loot")]
fn loot_index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../loot_index.html"))
}

#[get("/loot_tables.js")]
fn loot_js() -> RawJavaScript<&'static str> {
    RawJavaScript(include_str!("../loot_tables.js"))
}

#[get("/loot_tables_bg.wasm")]
fn loot_wasm() -> (ContentType, &'static [u8]){
    (ContentType::WASM, include_bytes!("../loot_tables_bg.wasm"))
}

#[get("/textonlyindex")]
fn index_as_text() -> &'static str {
    "Hello.  Welcome to the least user-friendly temperature convertor on the web.

    GET https://jh-hello-shuttle.shuttleapp.rs/ftoc/<farenheit> - returns temperature in Celsius

    GET https://jh-hello-shuttle.shuttleapp.rs/ctof/<celsuis> - returns temperature in Farenheit

    Now with a different api and added support for converting kelvin!

    GET https://jh-hello-shuttle.shuttleapp.rs/<from>/<to>/<from_value>

    where <from> and <to> must be exactly one of 'celsius', 'farenheit' or 'kelvin'
    (or 'c', 'f', 'k' if you don't like typing)
    and <from_value> is the known temperature (can include decimal point)

    Example:

    https://jh-hello-shuttle.shuttleapp.rs/farenheit/celsius/451

    And finally it will refuse if you ask it to convert a temperature that is below absolute zero.

    Use at your own risk.  Enjoy!"
}

#[get("/ctof/<celsius>")]
fn c_to_f(celsius: f32) -> Option<String> {
    Option::from(ctof(celsius).to_string())
}

#[get("/ftoc/<farenheit>")]
fn f_to_c(farenheit: f32) -> Option<String> {
    Option::from(ftoc(farenheit).to_string())
}

#[get("/<from>/<to>/<input>")]
fn convert_temperature(from: Scale, to: Scale, input: f32) -> Option<String> {

    let result = if Scale::below_minimum(&from, &input) {
        format!("Nothing can be {} {:?}, at least not in this universe. :-)", &input, &from)
    } else {
        convert(from, to, input).to_string()
    };

    Option::from(result)
}

/*
TODOs

get rid of old api
integrate minimum check and give a sensible  error.
try out maud https://maud.lambda.xyz/web-frameworks.html

 */

#[shuttle_service::main]
async fn rocket() -> Result<Rocket<Build>, shuttle_service::Error> {
    let rocket = rocket::build()
        .register("/duff", catchers![just_500])
        .register("/", catchers![not_found, just_500])
        .mount("/", routes![index, c_to_f, f_to_c, convert_temperature, index_as_text])
        .mount("/", routes![loot_index, loot_js, loot_wasm])
        ;

    Ok(rocket)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn c_to_f_works() {
        // 10 C = 50 F.
        let result = c_to_f(10.0);
        assert_eq!(result, Option::Some(String::from("50")));
    }

    #[test]
    fn f_to_c_works() {
        // 50 F = 10 C.
        let result = f_to_c(50.0);
        assert_eq!(result, Option::Some(String::from("10")));
    }

    #[test]
    fn ctof_works() {
        assert_eq!(ctof(10.0), 50.0)
    }

    #[test]
    fn ftoc_works() {
        assert_eq!(ftoc(50.0), 10.0)
    }
}
