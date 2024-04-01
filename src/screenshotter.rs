use chrono::prelude::*;
use screenshots::Screen;
use std::fs;
use std::time::Instant;

pub fn generate_print() {
  let screens = Screen::all().unwrap();

  let start = Instant::now();
  // Log a message
  println!("Calling prints");

  // Get current date
  let local_now = Local::now();
  let formatted_date = local_now.format("%Y-%m-%d").to_string();

  // Create directory for images if it doesn't exist
  let dir_path = format!("images/{}", formatted_date);
  fs::create_dir_all(&dir_path).unwrap();

  for screen in &screens {
      println!("capturer {:?}", screen);

      let formatted_time = local_now.format("%H-%M-%S").to_string();
      let formatted_date_time = format!("{}_{}", formatted_date, formatted_time);

      let image = screen.capture().unwrap();
      image
          .save(format!("{}/{}.png", dir_path, formatted_date_time))
          .unwrap();
  }

  println!("Capt: {:?}", start.elapsed());
}