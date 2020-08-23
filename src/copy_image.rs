#[cfg(not(target_os = "macos"))]
use anyhow::anyhow;
use anyhow::Result;
use image::io::Reader;
use image::DynamicImage;
use image::ImageFormat::Png;
use tempfile::NamedTempFile;
use tokio::process::Command;

/// Converts a clipboard image to image::DynamicImage
/// # Example
/// ```rust
/// use anyhow::Result;
///
/// async fn example() -> Result<()> {
///     let img = imboard::copy_image::from_clipboard().await?;
///     Ok(())
/// }
/// ```
#[cfg(target_os = "macos")]
pub async fn from_clipboard() -> Result<DynamicImage> {
    // Create a temporary file for storing clipboard image
    let temp_file = NamedTempFile::new()?;
    let temp_file_path = temp_file.path().to_str().unwrap();

    //  Copy clipboard image to a temporary file
    let script_to_copy = &format!(
        "write (the clipboard as «class PNGf») to (open for access \"{}\" with write permission)",
        temp_file_path
    );
    Command::new("osascript")
        .args(&["-e", script_to_copy])
        .spawn()?
        .await?;

    // Convert temporary file image to image::DynamicImage
    let img = Reader::open(temp_file_path)?
        .with_guessed_format()?
        .decode()?;

    // Delete temporary file
    temp_file.close()?;

    Ok(img)
}

#[cfg(not(target_os = "macos"))]
pub fn from_clipboard() -> Result<DynamicImage> {
    Err(anyhow!(
        "This crate does not support any OS other than macOS"
    ))
}

/// Copies the argument img to the clipboard
/// # Example
/// ```rust
/// use anyhow::Result;
///
/// async fn example() -> Result<()> {
///     let img = image::open("path/to/image")?;
///     imboard::copy_image::to_clipboard(img).await?;
///     Ok(())
/// }
/// ```
#[cfg(target_os = "macos")]
pub async fn to_clipboard(img: DynamicImage) -> Result<()> {
    // Create a temporary file to save image to the clipboard
    let temp_file = NamedTempFile::new()?;
    let temp_file_path = temp_file.path().to_str().unwrap();

    // Save the argument img to a temporary file as png
    img.save_with_format(temp_file_path, Png).unwrap();

    // Copy temporary file image to the clipboard
    let script_to_copy = &format!(
        "set the clipboard to (read \"{}\" as «class PNGf»)",
        temp_file_path
    );
    Command::new("osascript")
        .args(&["-e", script_to_copy])
        .spawn()?
        .await?;

    // Delete temporary file
    temp_file.close()?;

    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn to_clipboard() -> Result<()> {
    Err(anyhow!(
        "This crate does not support any OS other than macOS"
    ))
}
