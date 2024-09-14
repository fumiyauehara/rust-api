use rocket::{get, State};
use rocket::response::stream::{Event, EventStream};
use crate::api::models::app_state::AppState;

#[get("/sse")]
pub async fn sse(state: &State<AppState>) -> EventStream![] {
    let mut rx = state.sse_sender.subscribe();
    EventStream! {
        loop {
            match rx.recv().await {
                Ok(event) => yield Event::data(event),
                Err(_) => break,
            }
        }
    }
}
