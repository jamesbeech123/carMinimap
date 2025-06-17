// src/renderer.rs

use image::{DynamicImage, ImageReader, Rgba, RgbaImage, ImageBuffer};
use std::path::Path;

use crate::map;



/// Combines map tiles, centers on the GPS position, and draws a player icon.
/// 
/// # Arguments
/// * `tiles` - Tile coordinates.
/// * `tile_size` - Size of each tile.
/// * `zoom` - Zoom level.
/// * `gps_latlon` - (lat, lon) of the player.
/// * `display_size` - (width, height) of the output image.
/// 
/// # Returns
/// Centered map image with player icon.
pub fn render_centered_map(
    tiles: &[(i32, i32)],
    tile_size: u32,
    zoom: u32,
    gps_latlon: (f64, f64),
    display_size: (u32, u32),
) -> RgbaImage {
    // Combine tiles as before
    let grid = (tiles.iter().map(|(x, _)| x).max().unwrap() - tiles.iter().map(|(x, _)| x).min().unwrap() + 1) as u32;
    let mut canvas = RgbaImage::new(grid * tile_size, grid * tile_size);

    let min_x = tiles.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = tiles.iter().map(|(_, y)| *y).min().unwrap();

    for &(x, y) in tiles {
        let tile_path = crate::map::tile_path(x as u32, y as u32, zoom as u8);
        if let Ok(tile_img) = load_tile(tile_path.as_path()) {
            let dx = ((x - min_x) as u32) * tile_size;
            let dy = ((y - min_y) as u32) * tile_size;
            image::imageops::overlay(&mut canvas, &tile_img, dx.into(), dy.into());
        }
    }

    // Find GPS pixel in the combined image
    let (gps_px, gps_py) = map::latlon_to_pixel(gps_latlon.0, gps_latlon.1, zoom as u8);
    let top_left_tile_px = (
        (min_x as f64) * tile_size as f64,
        (min_y as f64) * tile_size as f64,
    );
    let gps_in_canvas = (
        (gps_px - top_left_tile_px.0) as i32,
        (gps_py - top_left_tile_px.1) as i32,
    );

    // Crop so GPS is at center
    let (disp_w, disp_h) = display_size;
    let crop_x = (gps_in_canvas.0 - (disp_w as i32) / 2).max(0) as u32;
    let crop_y = (gps_in_canvas.1 - (disp_h as i32) / 2).max(0) as u32;
    let cropped = image::imageops::crop_imm(&canvas, crop_x, crop_y, disp_w, disp_h).to_image();

    // Draw player icon at center
    let mut final_img = cropped.clone();
    let center = (disp_w / 2, disp_h / 2);
    draw_player_icon(&mut final_img, center);

    final_img
}

/// Draws a simple player icon (red circle) at the given position.
fn draw_player_icon(img: &mut RgbaImage, pos: (u32, u32)) {
    let (cx, cy) = pos;
    let radius = 8;
    let color = Rgba([255, 0, 0, 255]);
    for dx in -(radius as i32)..=(radius as i32) {
        for dy in -(radius as i32)..=(radius as i32) {
            if dx*dx + dy*dy <= (radius as i32)*(radius as i32) {
                let x = cx as i32 + dx;
                let y = cy as i32 + dy;
                if x >= 0 && y >= 0 && x < img.width() as i32 && y < img.height() as i32 {
                    img.put_pixel(x as u32, y as u32, color);
                }
            }
        }
    }
}

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
    log::debug!("Successfully loaded image.");
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
pub fn combine_images(
    tiles: &[(i32, i32)],
    tile_size: u32,
    zoom: u32,
) -> image::RgbaImage {
    let grid = (tiles.iter().map(|(x, _y)| x).max().unwrap() - tiles.iter().map(|(x, _y)| x).min().unwrap() + 1) as u32;
    let mut canvas = image::RgbaImage::new(grid * tile_size, grid * tile_size);

    let min_x = tiles.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = tiles.iter().map(|(_, y)| *y).min().unwrap();

    for &(x, y) in tiles {
        let tile_path = crate::map::tile_path(x as u32, y as u32, zoom as u8);
        if let Ok(tile_img) = load_tile(tile_path.as_path()) {
            let dx = ((x - min_x) as u32) * tile_size;
            let dy = ((y - min_y) as u32) * tile_size;
            image::imageops::overlay(&mut canvas, &tile_img, dx.into(), dy.into());
        }
    }

    canvas
}




