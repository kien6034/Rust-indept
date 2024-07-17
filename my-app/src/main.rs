// my-app/src/main.rs

use my_app_macros::IntoStringHashMap;
use std::collections::HashMap;

#[derive(IntoStringHashMap)]
pub struct User {
    username: String,
    first_name: String,
    last_name: String,
}

fn main() {
    let user = User {
        username: "username".to_string(),
        first_name: "First".to_string(),
        last_name: "Last".to_string(),
    };

    let hash_map = HashMap::<String, String>::from(user);

    dbg!(hash_map);
}
