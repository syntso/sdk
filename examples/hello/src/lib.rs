use serde::Deserialize;
use synt_sdk::serde_json::{Value, json};
use synt_sdk::{Error, Extension, Invocation, register_extension};

struct Hello;

#[derive(Deserialize)]
struct Greet {
    #[serde(default)]
    name: Option<String>,
}

impl Extension for Hello {
    fn run(invocation: &Invocation) -> Result<Value, Error> {
        if invocation.action() != "greet" {
            return Err(Error::unknown_action(invocation.action()));
        }

        let greet: Greet = invocation.params()?;
        let name = greet.name.as_deref().unwrap_or("world");

        Ok(json!({ "greeting": format!("Hello, {name}!") }))
    }
}

register_extension!(Hello);
