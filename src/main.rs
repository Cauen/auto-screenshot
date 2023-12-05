use chrono::prelude::*;
use screenshots::Screen;
use std::time::Instant;

fn main() {
    let screens = Screen::all().unwrap();

    // Run the loop forever
    loop {
        let start = Instant::now();
        // Log a message
        println!("Calling prints");
        for screen in screens {
            println!("capturer {screen:?}");
            let utc_now = Utc::now();
            let formatted_date_time = utc_now.format("%d-%y-%m-%H-%M-%S").to_string();

            let image = screen.capture().unwrap();
            image
                .save(format!("images/{}.png", formatted_date_time))
                .unwrap();
        }

        // Sleep for a while before logging the next message
        std::thread::sleep(std::time::Duration::from_secs(60));
        println!("Capt: {:?}", start.elapsed());
    }
}
