#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    portfolio_backend::rocket().launch().await?;
    Ok(())
}
