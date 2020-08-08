extern crate tempfile;
extern crate anyhow;

use tempfile::NamedTempFile;
use anyhow::Result;

#[cfg(target_os = "macos")]
pub fn copy_image_from_clipboard() -> Result<()> {
    let file = NamedTempFile::new()?;
    println!("{:?}", file.path());
    file.close()?;
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn copy_image_from_clipboard() -> Result<()> {
    Ok(())
}
