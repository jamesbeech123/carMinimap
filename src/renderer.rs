// src/renderer.rs

use image::{DynamicImage, ImageReader};
use show_image::{create_window, ImageInfo, ImageView};
use std::path::Path;

pub fn load_tile(tile: &Path) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    if !tile.exists() {
        return Err(format!("Tile image not found at path: {}", tile.display()).into());
    }

    let image = ImageReader::open(tile)?.decode()?;
    println!("[DEBUG] Successfully loaded image.");
    Ok(image)
}

pub fn render_tile(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let image = load_tile(path)?;

    // Convert to RGB8 for display
    let rgb_image = image.to_rgb8();
    let image_view = ImageView::new(
        ImageInfo::rgb8(rgb_image.width(), rgb_image.height()),
        &rgb_image,
    );

    let window = create_window("Tile Viewer", Default::default())?;
    window.set_image("tile", image_view)?;
    println!("Press Esc or close the window to exit.");
    window.wait_until_destroyed()?;

    Ok(())
}
