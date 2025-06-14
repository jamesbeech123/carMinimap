use image::{RgbaImage};
use show_image::{create_window, ImageInfo, ImageView};

mod map;
mod renderer;

#[tokio::main]
async fn main() {
    // Get current gps position
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

    // Compose all tiles into one image
    
    let grid = (tiles.iter().map(|(_x, _y)| _x).max().unwrap() - tiles.iter().map(|(_x, _y)| _x).min().unwrap() + 1) as u32;
    let mut canvas = RgbaImage::new(grid * tile_size, grid * tile_size);

    let min_x = tiles.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = tiles.iter().map(|(_, y)| *y).min().unwrap();

    for &(x, y) in &tiles {
        let tile_path = map::tile_path(x, y, zoom);
        if let Ok(tile_img) = renderer::load_tile(tile_path.as_path()) {
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

    // Wait until the window is closed
    
}