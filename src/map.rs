use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

// use crate::renderer;



/// Fetches a map tile from disk or downloads it from OpenStreetMap if not found.
///
/// # Arguments
///
/// * `x` - The x-coordinate of the tile.
/// * `y` - The y-coordinate of the tile.
/// * `zoom` - The zoom level of the tile.
///
/// # Returns
///
/// A `Result` containing the tile image bytes on success, or an error if the operation fails.
pub async fn fetch_tile(x: u32, y: u32, zoom: u8) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let path = format!("tiles/{}/{}/{}.png", zoom, x, y);
    let tile_path = Path::new(&path);

    println!("[DEBUG] Tile path: {}", tile_path.display());

    //Check if tile is downloaded
    if tile_path.exists() {
        println!("[DEBUG] Tile found on disk. Loading from {}", tile_path.display());
        let data = fs::read(tile_path)?;
        println!("[DEBUG] Successfully loaded tile from disk.");
        return Ok(data);
    }

    //Attempt to get tile from openstreetmap
    let url = format!("https://a.tile.openstreetmap.fr/osmfr/{}/{}/{}.png", zoom, x, y);
    println!("[DEBUG] Tile not found on disk. Fetching from URL: {}", url);

    //Build request
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "MyTileFetcher/1.0 (contact@example.com)") // Replace or obfuscate if publishing
        .send()
        .await?;

    println!("[DEBUG] HTTP response received.");

    if !response.status().is_success() {
        println!("[ERROR] Failed to fetch tile: HTTP {}", response.status());
        return Err(format!("Failed to fetch tile: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;
    println!("[DEBUG] Downloaded {} bytes.", bytes.len());

    fs::create_dir_all(tile_path.parent().unwrap())?;
    println!("[DEBUG] Created directories up to {}", tile_path.parent().unwrap().display());

    let mut file = fs::File::create(tile_path)?;
    file.write_all(&bytes)?;
    println!("[DEBUG] Saved tile to disk at {}", tile_path.display());
    

    

    Ok(bytes.to_vec())
}



/// Converts a latitude and longitude to tile x and y coordinates at a given zoom level.
///
/// # Arguments
///
/// * `lat` - The latitude in degrees.
/// * `lon` - The longitude in degrees.
/// * `zoom` - The zoom level (0–19).
///
/// # Returns
///
/// A tuple `(x, y)` representing the tile coordinates.
pub fn gps_to_tile(lat: f64, lon: f64, zoom: u8) -> (u32, u32) {
    println!("[DEBUG] Converting GPS to tile: lat={}, lon={}, zoom={}", lat, lon, zoom);

    let lat_rad = lat.to_radians();
    let n = 2u32.pow(zoom as u32) as f64;

    let x = ((lon + 180.0) / 360.0 * n).floor();
    let y = ((1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / std::f64::consts::PI) / 2.0 * n).floor();

    let tile_x = x as u32;
    let tile_y = y as u32;

    println!("[DEBUG] Computed tile coordinates: x={}, y={}", tile_x, tile_y);

    (tile_x, tile_y)
}


/// Gets the surrounding tiles for a given GPS coordinate and zoom level.
/// This includes the center tile and all adjacent tiles in a 3x3 grid.
/// 
/// # Arguments
/// * `lat` - The latitude in degrees.
/// * `lon` - The longitude in degrees.
/// * `zoom` - The zoom level (0–19).
/// 
/// # Returns
/// 
/// A vector of tuples containing the x and y coordinates of the surrounding tiles.
pub fn get_surrounding_tiles(lat: f64, lon: f64, zoom: u8) -> Vec<(u32, u32)> {
    let (x_tile, y_tile) = gps_to_tile(lat, lon, zoom);
    println!("[DEBUG] Center tile coordinates: x={}, y={}", x_tile, y_tile);

    let mut tiles = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            let new_x = (x_tile as i32 + dx).max(0) as u32;
            let new_y = (y_tile as i32 + dy).max(0) as u32;
            tiles.push((new_x, new_y));
        }
    }

    println!("[DEBUG] Surrounding tiles: {:?}", tiles);
    tiles
}


/// Constructs the file path for a tile image based on its coordinates and zoom level.
/// 
/// # Arguments
/// * `x` - The x-coordinate of the tile.
/// * `y` - The y-coordinate of the tile.
/// * `zoom` - The zoom level of the tile.
/// 
/// # Returns
/// 
/// A `PathBuf` representing the file path to the tile image.
pub fn tile_path(x: u32, y: u32, zoom: u8) -> PathBuf {
    let path = format!("tiles/{}/{}/{}.png", zoom, x, y);
    PathBuf::from(path)
}





