#[get("/health")]
pub async fn health() -> &'static str {
    "not dead"
}
