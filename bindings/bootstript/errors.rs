use std::error::Error;
use std::fmt::{Debug, Formatter, Display};


pub struct MyError {

}

pub type Result<T> = std::result::Result<T, MyError>;

impl Debug for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for MyError {

}