use chrono::prelude::*;
use screenshots::Screen;
use std::fs;
use std::time::Instant;

pub fn start_screen_shotter() {
    let screens = Screen::all().unwrap();
    fs::create_dir_all("images").unwrap();

    // Run the loop forever
    loop {
        let start = Instant::now();
        // Log a message
        println!("Calling prints");
        for screen in &screens {
            println!("capturer {:?}", screen);
            let utc_now = Utc::now();
            let formatted_date_time = utc_now.format("screenshot-%d-%m-%Y--%H-%M-%SZ").to_string();

            let image = screen.capture().unwrap();
            image
                .save(format!("images/{}.png", formatted_date_time))
                .unwrap();
        }

        // Sleep for a while before logging the next message
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("Capt: {:?}", start.elapsed());
    }
}
