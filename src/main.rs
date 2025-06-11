

mod map;
mod renderer;

#[tokio::main]
async fn main() {
    //Get current gps position
    let lat = 25.2048;
    let lon = 55.2708;
    let zoom = 15;

    //Convert gps to map tile and pixel coordinates
    let (x_tile, y_tile) = map::gps_to_tile(lat, lon, zoom);
    println!("Tile X: {}, Tile Y: {}, Zoom: {}", x_tile, y_tile, zoom);

    match map::fetch_tile(x_tile, y_tile, zoom).await {
        Ok(_) => println!("✅ Tile successfully fetched or loaded."),
        Err(e) => eprintln!("❌ Error fetching tile: {}", e),
    }

    //Load surrounding map tiles
    //Render Map Centered on user
    //Draw Overlays
    //Present Frame

}