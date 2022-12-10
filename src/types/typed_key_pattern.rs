use serde::{Deserialize, Serialize};
use std::{result::{Result}, error::Error};
use serde_json::{Value};

pub enum MyError {
    GoingWild
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Foo {
    value: u64
}

#[derive(Deserialize, Debug)]
struct Config {
    foo: Foo
}
fn parse_config(data: &str){
    let config: Config = serde_json::from_str(data).unwrap();
    println!("{:?}", config);
    //Ok(config)
}

pub fn run() {
    let data = r#"
        {
            "foo": {
                "value": 10
            }
           
        }"#;

    parse_config(data);
}