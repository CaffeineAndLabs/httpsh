#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket_contrib::{JSON, Value};
use std::process::Command;

// Representation of a system command
// Args are optionals
#[derive(Serialize, Deserialize)]
struct Cmd {
    name: String,
    args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct ResultCmd {
    stdout: String,
    stderr: String,
    exit_status: i32,
}

#[get("/ping")]
fn ping() -> JSON<Value> {
    JSON(json!({
        "status": "ok",
        "content": "pong"
    }))
}

#[post("/cmd/exec", format = "Application/json", data = "<body>")]
fn cmd_exec(body: JSON<Cmd>) -> JSON<ResultCmd> {
    // Consumes the JSON wrapper and returns the wrapped item
    let cmd = body.into_inner();

    // Match if there is args in POSTed command or not
    let output = match cmd.args {
        Some(args) => {
            Command::new(cmd.name)
                .args(args)
                .output()
                .expect("Failed to execute command")
        }
        None => {
            Command::new(cmd.name)
                .output()
                .expect("Failed to execute command")
        }
    };

    let res = ResultCmd {
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
        exit_status: output.status.code().unwrap(),
    };

    JSON(res)
}

#[error(404)]
fn not_found() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![ping, cmd_exec])
        .catch(errors![not_found])
        .launch();
}
