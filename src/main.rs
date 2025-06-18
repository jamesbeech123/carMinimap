use std::sync::mpsc;
use show_image::{create_window, ImageInfo, ImageView, run_context};
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};

mod map;
mod renderer;
mod gps;

#[tokio::main]
async fn main() {
    //Enable debug messages if the environment variable is set
    env_logger::init();

    // Channel for sending images from async task to UI
    let (tx, rx) = mpsc::channel();

    // Get current GPS position
    let gps_data = Arc::new(Mutex::new((25.1019, 55.2394))); // fallback/default
    let gps_port = "COM3"; // Adjust this to your GPS device's port
    gps::spawn_gps_reader(gps_port, gps_data.clone());

    // Spawn async task to fetch and compose images
    tokio::spawn(async move {
        let mut prev_gps = *gps_data.lock().unwrap();
        loop {
            let curr_gps = *gps_data.lock().unwrap();
            for step in 0..10 {
                let lat = prev_gps.0 + (curr_gps.0 - prev_gps.0) * (step as f64) / 10.0;
                let lon = prev_gps.1 + (curr_gps.1 - prev_gps.1) * (step as f64) / 10.0;

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
                // Provide the missing arguments as appropriate for your use case
                let center = (lat, lon);
                let canvas = renderer::render_centered_map(
                    &tiles_i32,
                    tile_size,
                    zoom.into(),
                    center,
                    (512, 512),
                );

                // 5. Send the image to the UI thread
                if tx.send(canvas).is_err() {
                    break; // UI closed
                }

                sleep(Duration::from_millis(20)).await;
            }
            prev_gps = curr_gps;
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