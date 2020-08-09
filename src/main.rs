use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let img = imboard::copy_image_from_clipboard().await?;
    img.to_rgba().save("sample.png").unwrap();
    Ok(())
}
