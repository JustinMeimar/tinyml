use std::fs::File;
use std::path::Path;
use std::io::Read;

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    
    let path = Path::new(path);    
    let mut file = File::open(path)?;
    let mut content = String::new();
    
    file.read_to_string(&mut content)?; 
    Ok(content)
}
