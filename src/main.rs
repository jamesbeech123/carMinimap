mod map;
mod renderer;

#[tokio::main]
async fn main() {
    // Get current gps position, currently hardcoded
    let lat = 25.2048;
    let lon = 55.2708;
    let zoom = 17;
    let tile_size = 256; 
    
    // Get all surrounding tiles (including center)
    let tiles = map::get_surrounding_tiles(lat, lon, zoom);

    // Fetch all tiles asynchronously
    for &(x, y) in &tiles {
        let _ = map::fetch_tile(x, y, zoom).await;
    }

    // Convert tiles from Vec<(u32, u32)> to Vec<(i32, i32)>
    let tiles_i32: Vec<(i32, i32)> = tiles.iter().map(|&(x, y)| (x as i32, y as i32)).collect();

    renderer::combine_images(&tiles_i32, tile_size, zoom.into());

    

    
    // Wait until the window is closed
    
}