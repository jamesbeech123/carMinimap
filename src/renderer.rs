// src/renderer.rs

use image::{DynamicImage, ImageReader};
use std::path::Path;


/// Loads a tile image from the specified path.
/// 
/// # Arguments
/// * `tile` - The path to the tile image.
/// 
/// # Returns
/// 
/// A `Result` containing the loaded image on success, or an error if the operation fails.
pub fn load_tile(tile: &Path) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    if !tile.exists() {
        return Err(format!("Tile image not found at path: {}", tile.display()).into());
    }

    let image = ImageReader::open(tile)?.decode()?;
    println!("[DEBUG] Successfully loaded image.");
    Ok(image)
}


