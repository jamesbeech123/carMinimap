use std::sync::mpsc;
use show_image::{create_window, ImageInfo, ImageView, run_context};
use tokio::time::{sleep, Duration};

mod map;
mod renderer;

#[tokio::main]
async fn main() {
    //Enable debug messages if the environment variable is set
    env_logger::init();

    // Channel for sending images from async task to UI
    let (tx, rx) = mpsc::channel();

    // Spawn async task to fetch and compose images
    tokio::spawn(async move {
        loop {
            // 1. Get current GPS position (update if needed)
            let lat = 25.1019;
            let lon = 55.2394;
            let zoom = 17;
            let tile_size = 256;

            // 2. Get surrounding tiles
            let tiles = map::get_surrounding_tiles(lat, lon, zoom);

            // 3. Fetch all tiles asynchronously
            for &(x, y) in &tiles {
                let _ = map::fetch_tile(x, y, zoom).await;
            }

            // 4. Render the map (compose image)
            let tiles_i32: Vec<(i32, i32)> = tiles.iter().map(|&(x, y)| (x as i32, y as i32)).collect();
            let canvas = renderer::combine_images(&tiles_i32, tile_size, zoom.into());

            // 5. Send the image to the UI thread
            if tx.send(canvas).is_err() {
                break; // UI closed
            }

            sleep(Duration::from_secs(5)).await;
        }
    });

    // UI thread: receive and display images
    run_context(move || {
        let window = create_window("Map", Default::default()).unwrap();
        for canvas in rx {
            window.set_image(
                "map",
                ImageView::new(ImageInfo::rgba8(canvas.width(), canvas.height()), &canvas),
            ).unwrap();
        }
        // The closure never returns, so no need to return Ok(()) or call .unwrap()
    });
}