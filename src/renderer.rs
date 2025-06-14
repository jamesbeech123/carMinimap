// src/renderer.rs

use image::{DynamicImage, ImageReader};
use std::path::Path;
use crate::map;
use show_image::{create_window, ImageView, ImageInfo};



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


/// Combines multiple map tiles into a single image and displays it in a window.
/// 
/// # Arguments
/// * `tiles` - A slice of tuples containing the x and y coordinates of the tiles.
/// * `tile_size` - The size of each tile in pixels.
/// * `zoom` - The zoom level of the tiles.
/// 
/// # Returns
///     
/// This function does not return a value. It displays the composed image in a window.
pub fn combine_images(tiles: &[(i32, i32)],tile_size: u32,zoom: u32,) {
    // Compose all tiles into one image
    let grid = (tiles.iter().map(|(x, _y)| x).max().unwrap() - tiles.iter().map(|(x, _y)| x).min().unwrap() + 1) as u32;
    let mut canvas = image::RgbaImage::new(grid * tile_size, grid * tile_size);

    let min_x = tiles.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = tiles.iter().map(|(_, y)| *y).min().unwrap();

    for &(x, y) in tiles {
        let tile_path = map::tile_path(x as u32, y as u32, zoom as u8);
        if let Ok(tile_img) = load_tile(tile_path.as_path()) {
            let dx = ((x - min_x) as u32) * tile_size;
            let dy = ((y - min_y) as u32) * tile_size;
            image::imageops::overlay(&mut canvas, &tile_img, dx.into(), dy.into());
        }
    }

    // Show the composed image in a single window
    show_image::run_context(move || -> Result<(), Box<dyn std::error::Error>> {
        let window = create_window("Map", Default::default()).unwrap();
        window.set_image(
            "map",
            ImageView::new(ImageInfo::rgba8(canvas.width(), canvas.height()), &canvas),
        ).unwrap();
        window.wait_until_destroyed().unwrap();
        Ok(())
    });
}




