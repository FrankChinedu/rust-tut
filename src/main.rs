use std::fs;
use std::path::Path;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::rename(Path::new("temp.txt"), Path::new("final.txt"))?;
    Ok(())
}
