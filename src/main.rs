use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use tray_item::{IconSource, TrayItem};

mod screenshotter;
mod utils {
    pub mod logs;
}

enum Message {
    Quit,
    Start,
    Stop,
    OpenFolder,
}

fn main() {
    let mut tray = TrayItem::new("Auto Screenshot", IconSource::Resource("stopped-icon")).unwrap();

    // tray.add_label("Options").unwrap();

    let (tx, rx) = mpsc::sync_channel(1);
    let printing = Arc::new(Mutex::new(false));
    let printing_clone = Arc::clone(&printing);

    let start_tx = tx.clone();
    tray.add_menu_item("Start", move || {
        start_tx.send(Message::Start).unwrap();
    })
    .unwrap();

    let stop_tx = tx.clone();
    tray.add_menu_item("Stop", move || {
        stop_tx.send(Message::Stop).unwrap();
    })
    .unwrap();

    tray.inner_mut().add_separator().unwrap();

    let open_tx = tx.clone();
    tray.add_menu_item("Open Folder", move || {
        open_tx.send(Message::OpenFolder).unwrap();
    })
    .unwrap();

    tray.inner_mut().add_separator().unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    utils::logs::log_message("Initialized");

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                utils::logs::log_message("Quit");
                break;
            }
            Ok(Message::Start) => {
                utils::logs::log_message("Start");
                let mut printing = printing.lock().unwrap();
                *printing = true;
                tray.set_icon(IconSource::Resource("started-icon")).unwrap();
                let printing_clone = Arc::clone(&printing_clone);
                thread::spawn(move || {
                    while *printing_clone.lock().unwrap() {
                        utils::logs::log_message("Spawn thread");
                        screenshotter::generate_print();
                        thread::sleep(Duration::from_secs(60));
                    }
                });
            }
            Ok(Message::Stop) => {
                utils::logs::log_message("Stop");
                let mut printing = printing.lock().unwrap();
                *printing = false;
                tray.set_icon(IconSource::Resource("stopped-icon")).unwrap();
            }
            Ok(Message::OpenFolder) => {
                utils::logs::log_message("Openning Folder");
                utils::logs::log_message("Owpa");
                let path = ".";
                match open::that(path) {
                    Ok(()) => {
                        let ok_message = format!("Opened '{}' successfully.", path);
                        utils::logs::log_message(&ok_message)
                    }
                    Err(err) => {
                        let error_message =
                            format!("An error occurred when opening '{}': {}.", path, err);
                        utils::logs::log_message(error_message.as_str())
                    }
                }
            }
            _ => {
                utils::logs::log_message("Other case");
            }
        }
    }
}
