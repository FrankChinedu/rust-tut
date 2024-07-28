use std::fs;
use std::path::Path;

mod as_ref;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::rename(Path::new("temp.txt"), Path::new("final.txt"))?;
    Ok(())
}
