extern crate anyhow;
extern crate tempfile;

use anyhow::Result;
use tempfile::NamedTempFile;

#[cfg(target_os = "macos")]
pub fn copy_image_from_clipboard() -> Result<()> {
    let file = NamedTempFile::new()?;
    let file_name = format!("{:?}", file.path());
    println!("{}", file_name);
    file.close()?;
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn copy_image_from_clipboard() -> Result<()> {
    Ok(())
}
