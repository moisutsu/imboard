use anyhow::Result;

fn main() -> Result<()> {
    imboard::copy_image_from_clipboard()?;
    Ok(())
}
