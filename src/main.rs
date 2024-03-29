#[macro_use]
extern crate rocket;

use std::path::PathBuf;
use rocket::request::FromParam;
use rocket::fs::{FileServer, relative};

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
        if param.eq_ignore_ascii_case("celsius") || param.eq_ignore_ascii_case("c") {
            Ok(Scale::Celsius)
        } else if param.eq_ignore_ascii_case("farenheit") || param.eq_ignore_ascii_case("f") {
            Ok(Scale::Farenheit)
        } else if param.eq_ignore_ascii_case("kelvin") || param.eq_ignore_ascii_case("k") {
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

            Scale::Kelvin => { input }
            Scale::Celsius => { ktoc(input) }
        }
    }
}



// instructions on how to build the game and compile into wasm
// are here: https://hands-on-rust.com/2021/11/06/run-your-rust-games-in-a-browser-hands-on-rust-bonus-content/


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
        format!(
            "Nothing can be {} {:?}, at least not in this universe. :-)",
            &input, &from
        )
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

#[shuttle_runtime::main]
async fn rocket(#[shuttle_static_folder::StaticFolder] _static_folder: PathBuf) -> shuttle_rocket::ShuttleRocket {

    let rocket = rocket::build()
        .register("/duff", catchers![just_500])
        .register("/", catchers![not_found, just_500])
        .mount(
            "/",
            routes![c_to_f, f_to_c, convert_temperature],
        )
        //.mount("/", routes![loot_index, loot_js])
        // .mount("/", routes![loot_index, loot_js, loot_wasm])
        .mount("/", FileServer::from(relative!("static")))
        ;

    Ok(rocket.into())
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
