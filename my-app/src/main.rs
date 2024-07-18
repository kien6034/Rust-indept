// my-app/src/main.rs

use my_app_macros::{DeriveCustomModel, IntoHashMap};
use std::collections::HashMap;

#[derive(IntoHashMap)]
pub struct User {
    username: String,
    first_name: String,
    last_name: String,
}

#[derive(DeriveCustomModel)]
#[custom_model(model(
    name = "UserName",
    fields(first_name, last_name),
    extra_derives(IntoHashMap)
))]
#[custom_model(model(name = "UserInfo", fields(username, age), extra_derives(Debug)))]
pub struct User2 {
    username: String,
    first_name: String,
    last_name: String,
    age: u32,
}

fn main() {
    let user = User {
        username: "username".to_string(),
        first_name: "First".to_string(),
        last_name: "Last".to_string(),
    };

    let hash_map = HashMap::<String, String>::from(user);

    dbg!(hash_map);

    // This user info field is derived from the DeriveCustomModel macros
    let user_info = UserInfo {
        username: "username".to_string(),
        age: 27,
    };

    dbg!(user_info);
}
