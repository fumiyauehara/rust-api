use rocket::get;
use rocket_ws::{Message, Stream, WebSocket};

#[get("/index")]
pub fn index(ws: WebSocket) -> Stream!['static] {
    Stream! { ws =>
        for await message in ws {
            match message? {
                Message::Text(text) => {
                    yield Message::Text(format!("You said: {}", text));
                }
                Message::Binary(_) => {
                    yield "Binary data received".into();
                }
                _ => {}
            }
        }
    }
}