# Imboard

[![Crates.io](https://img.shields.io/crates/v/imboard.svg)](https://crates.io/crates/imboard)
[![Documentation](https://docs.rs/imboard/badge.svg)](https://docs.rs/imboard)
![License](https://img.shields.io/crates/l/imboard.svg)

This is a crate that allows you to easily exchange images with the clipboard.

Currently, only macOS is supported.

# Example

Save a clipboard image to a file as `clipboard.png`
```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let img = imboard::copy_image::from_clipboard().await?;
    img.to_rgba().save("clipboard.png").unwrap();
    Ok(())
}
```

Copy a file image to the clipboard
```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let img = image::open("examples/images/copy.png")?;
    imboard::copy_image::to_clipboard(img).await?;
    Ok(())
}
```
