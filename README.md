# Imboard

This is a crate that allows you to easily exchange images with the clipboard.

Currently, only macOS is supported.

# Example

Save the clipboard image to a file as `clipboard.png`
```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let img = imboard::copy_image_from_clipboard().await?;
    img.to_rgba().save("clipboard.png").unwrap();
    Ok(())
}
```

Copy a file's image to the clipboard
```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let img = image::open("examples/images/copy.png")?;
    imboard::copy_image_to_clipboard(img).await?;
    Ok(())
}
```
