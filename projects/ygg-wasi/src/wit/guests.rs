use super::*;

impl Guest for JsonHost {
    type JsonNumber = JsonNumber;
    type JsonString = JsonString;
    type JsonArray = JsonArray;
}

impl GuestJsonNumber for JsonNumber {
    fn get_rule() -> JsonRule {
        todo!()
    }
}

impl GuestJsonString for JsonString {
    fn get_rule() -> JsonRule {
        todo!()
    }
}

impl GuestJsonArray for JsonArray {
    fn get_rule() -> JsonRule {
        todo!()
    }

    fn get_item(&self) -> Result<Vec<Json>, ()> {
        todo!()
    }

    fn get_text(&self) -> Result<String, ()> {
        todo!()
    }

    fn get_span(&self) -> Result<String, ()> {
        todo!()
    }
}
