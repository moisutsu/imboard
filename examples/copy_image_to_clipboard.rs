use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let img = image::open("examples/images/copy.png")?;
    imboard::copy_image_to_clipboard(img).await?;
    Ok(())
}
