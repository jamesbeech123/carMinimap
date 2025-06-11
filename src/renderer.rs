use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::path::Path;


pub fn load_tile(tile: &Path) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    //println!("[DEBUG] Loading tile image from path: {}", tile);

    let path = Path::new(tile);
    if !path.exists() {
        return Err(format!("Tile image not found at path: ").into());
    }

    let image = ImageReader::open(path)?.decode()?;
    println!("[DEBUG] Successfully loaded image.");


    
    Ok(image)
}
