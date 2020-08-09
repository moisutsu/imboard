use anyhow::Result;
use image::io::Reader;
use image::DynamicImage;
use image::ImageFormat::Png;
use tempfile::NamedTempFile;
use tokio::process::Command;

/// A `copy_image_from_clipboard` converts a clipboard image to image::DynamicImage and returns it.
#[cfg(target_os = "macos")]
pub async fn copy_image_from_clipboard() -> Result<DynamicImage> {
    // Create a temporary file for storing clipboard image
    let temp_file = NamedTempFile::new()?;
    let temp_file_name = temp_file.path().to_str().unwrap();

    //  Copy clipboard image to a temporary file
    let script_to_copy = &format!(
        "write (the clipboard as «class PNGf») to (open for access \"{}\" with write permission)",
        temp_file_name
    );
    Command::new("osascript")
        .args(&["-e", script_to_copy])
        .spawn()?
        .await?;

    // Convert image from temporary file to <image::DynamicImage>
    let img = Reader::open(temp_file_name)?
        .with_guessed_format()?
        .decode()?;

    // Delete temporary file
    temp_file.close()?;

    Ok(img)
}

#[cfg(not(target_os = "macos"))]
pub fn copy_image_from_clipboard() -> Result<()> {
    Err("This crate does not support any OS other than macOS")
}

/// A `copy_image_to_clipboard` copies the argument img to the clipboard
#[cfg(target_os = "macos")]
pub async fn copy_image_to_clipboard(img: DynamicImage) -> Result<()> {
    // Create a temporary file to save image to the clipboard
    let temp_file = NamedTempFile::new()?;
    let temp_file_name = temp_file.path().to_str().unwrap();

    // Save the argument img to a temporary file as png
    img.save_with_format(temp_file_name, Png).unwrap();

    // Copy temporary file image to the clipboard
    let script_to_copy = &format!(
        "set the clipboard to (read \"{}\" as TIFF picture)",
        temp_file_name
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
pub fn copy_image_to_clipboard() -> Result<()> {
    Err("This crate does not support any OS other than macOS")
}
