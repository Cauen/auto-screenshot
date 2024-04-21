use chrono::prelude::*;
use screenshots::Screen;
use std::fs;
use std::time::Instant;

use crate::utils::logs::log_message;

pub fn generate_print() {
  let screens = Screen::all().unwrap();

  let start = Instant::now();
  // Log a message
  log_message("Calling prints");

  // Get current date
  let local_now = Local::now();
  let formatted_date = local_now.format("%Y-%m-%d").to_string();

  // Create directory for images if it doesn't exist
  let dir_path = format!("images/{}", formatted_date);
  fs::create_dir_all(&dir_path).unwrap();

  for screen in &screens {
      let capturer_formatted = format!("capturer {:?}", screen);
      log_message(&capturer_formatted);

      let formatted_time = local_now.format("%H-%M-%S").to_string();
      let formatted_date_time = format!("{}_{}", formatted_date, formatted_time);

      let image = screen.capture().unwrap();
      image
          .save(format!("{}/{}.png", dir_path, formatted_date_time))
          .unwrap();
  }

  let capturer_formatted = format!("Capt: {:?}", start.elapsed());
  log_message(&capturer_formatted);
}