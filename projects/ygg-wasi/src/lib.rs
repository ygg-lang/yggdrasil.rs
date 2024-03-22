mod ast;

wit_bindgen::generate!({
    world: "host",
});

struct JsonHost;

export!(JsonHost);
