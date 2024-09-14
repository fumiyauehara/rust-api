pub struct AppState {
    pub sse_sender: tokio::sync::broadcast::Sender<String>,
}