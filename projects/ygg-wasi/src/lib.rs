mod wit;

pub use crate::wit::JsonHost;

wit_bindgen::generate!({
    world: "host",
});

export!(JsonHost);
