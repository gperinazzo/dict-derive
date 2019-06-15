/*
 * Tests basic types usage
 */
use dict_derive::IntoPyObject;

#[derive(IntoPyObject)]
pub struct User {
    name: String,
    email: String,
    age: u16,
}

use std::option;

#[derive(IntoPyObject)]
pub struct OptionalUser {
    name: Option<String>,
    email: option::Option<String>,
    age: std::option::Option<u16>,
}

#[derive(IntoPyObject)]
pub struct Nested {
    users: Vec<User>,
    optional_user: Option<OptionalUser>,
}

fn main() {}
