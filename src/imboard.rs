extern crate anyhow;
extern crate image;
extern crate tempfile;
extern crate tokio;

use anyhow::Result;
use image::io::Reader;
use image::DynamicImage;
use tempfile::NamedTempFile;
use tokio::process::Command;

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
    Ok(())
}
