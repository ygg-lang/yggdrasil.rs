use crate::{
    exports::yggdrasil::json::ast::{Guest, GuestJsonArray, GuestJsonNumber, GuestJsonString, Json},
    JsonHost,
};

impl Guest for JsonHost {
    type JsonNumber = JsonNumber;
    type JsonString = JsonString;
    type JsonArray = JsonArray;
}

impl GuestJsonNumber for JsonNumber {}

impl GuestJsonString for JsonString {}
impl GuestJsonArray for JsonArray {
    fn item(&self) -> Result<Vec<Json>, ()> {
        todo!()
    }
}

pub struct JsonNumber {}

pub struct JsonString {}

pub struct JsonArray {}
