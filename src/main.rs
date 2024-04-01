use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};

mod screenshotter;

enum Message {
    Quit,
    Green,
    Red,
}

fn main() {
    let mut tray = TrayItem::new(
        "Auto Screenshot",
        IconSource::Resource("another-name-from-rc-file"),
    )
    .unwrap();

    tray.add_label("Options").unwrap();

    // tray.add_menu_item("Hello", || {
    //     println!("Hello!");
    // })
    // .unwrap();

    // tray.inner_mut().add_separator().unwrap();

    let (tx, rx) = mpsc::sync_channel(1);

    let green_tx = tx.clone();
    tray.add_menu_item("Start", move || {
        green_tx.send(Message::Green).unwrap();
    })
    .unwrap();

    let red_tx = tx.clone();
    tray.add_menu_item("Stop", move || {
        red_tx.send(Message::Red).unwrap();
    })
    .unwrap();

    tray.inner_mut().add_separator().unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break;
            }
            Ok(Message::Red) => {
                println!("Red");
                tray.set_icon(IconSource::Resource("another-name-from-rc-file"))
                    .unwrap();
            }
            Ok(Message::Green) => {
                println!("Green");
                tray.set_icon(IconSource::Resource("name-of-icon-in-rc-file"))
                    .unwrap();
                screenshotter::start_screen_shotter();
            }
            _ => {}
        }
    }
}
