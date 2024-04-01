use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use tray_item::{IconSource, TrayItem};

mod screenshotter;

enum Message {
    Quit,
    Start,
    Stop,
}

fn main() {
    let mut tray = TrayItem::new(
        "Auto Screenshot",
        IconSource::Resource("stopped-icon"),
    )
    .unwrap();

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

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    println!("Initialized");

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break;
            }
            Ok(Message::Start) => {
                println!("Start");
                let mut printing = printing.lock().unwrap();
                *printing = true;
                tray.set_icon(IconSource::Resource("started-icon")).unwrap();
                let printing_clone = Arc::clone(&printing_clone);
                thread::spawn(move || {
                    while *printing_clone.lock().unwrap() {
                        println!("printed");
                        screenshotter::generate_print();
                        thread::sleep(Duration::from_secs(60));
                    }
                });
            }
            Ok(Message::Stop) => {
                println!("Stop");
                let mut printing = printing.lock().unwrap();
                *printing = false;
                tray.set_icon(IconSource::Resource("stopped-icon")).unwrap();
            }
            _ => {}
        }
    }
}
