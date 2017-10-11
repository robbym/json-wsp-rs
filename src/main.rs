#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(wsp_methods_derive)]
#![allow(non_snake_case)]

#[macro_use]
extern crate wsp_type_derive;

extern crate serde_json;
use serde_json::{Value};
use serde_json::map::{Map};


trait WSPType {
    fn get_name() -> String;
    fn get_type() -> Value;
}

struct WSPAttachment (Vec<u8>);

trait WSPTypeMember {
    fn get_type_member() -> Value;
}

impl WSPTypeMember for bool {
    fn get_type_member() -> Value {
        Value::String(String::from("boolean"))
    }
}

impl WSPTypeMember for u8 {
    fn get_type_member() -> Value {
        Value::String(String::from("number"))
    }
}

impl WSPTypeMember for u16 {
    fn get_type_member() -> Value {
        Value::String(String::from("number"))
    }
}

impl WSPTypeMember for u32 {
    fn get_type_member() -> Value {
        Value::String(String::from("number"))
    }
}

impl WSPTypeMember for u64 {
    fn get_type_member() -> Value {
        Value::String(String::from("number"))
    }
}

impl WSPTypeMember for f32 {
    fn get_type_member() -> Value {
        Value::String(String::from("float"))
    }
}

impl WSPTypeMember for f64 {
    fn get_type_member() -> Value {
        Value::String(String::from("float"))
    }
}

impl WSPTypeMember for String {
    fn get_type_member() -> Value {
        Value::String(String::from("string"))
    }
}

impl WSPTypeMember for WSPAttachment {
    fn get_type_member() -> Value {
        Value::String(String::from("attachment"))
    }
}

impl<T> WSPTypeMember for Vec<T> where T: WSPTypeMember {
    fn get_type_member() -> Value {
        Value::Array(vec![T::get_type_member()])
    }
}


#[derive(WSPType)]
struct Group {
    group_id: u8,
    display_name: String,
    name: String,
    members: Vec<User>
}

#[derive(WSPType, WSPTypeMember)]
struct User {
    username: String,
    user_id: u32,
    mobile: String,
    age: u8,
    given_name: String,
    surname: String,
}

#[derive(WSPType)]
struct CreateUserResponse {
    user_id: u32,
    success: bool
}

trait WSPService {
    fn get_service() -> Value;
}

#[derive(WSPService)]
struct UserService;

trait WSPMethods {
    fn get_types() -> Value;
    fn get_methods() -> Value;
}

#[derive(WSPMethods)]
impl UserService {
    #[WSPMethod(User, Other)]
    fn listUsers(name_filter: String) -> Vec<User> {
        unimplemented!();
    }
    #[WSPMethod]
    fn listGroups() {

    }
    #[WSPMethod]
    fn createUser() {

    }
}

fn main() {
    println!("{}", UserService::get_service().to_string());
}
