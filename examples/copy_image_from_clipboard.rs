use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let img = imboard::copy_image::from_clipboard().await?;
    img.to_rgba().save("clipboard.png").unwrap();
    Ok(())
}
