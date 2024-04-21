![image](https://github.com/Cauen/auto-screenshot/assets/8796757/90485fa1-cb7f-4652-8a9f-c1f959f87b96)

# Rust Auto Screen Shotter

A 1MB binary to create a tray that can start/stop a auto screen shotter to take a fullscreen of every screen at every 60 seconds. 
Every file are stored at a folder with current date.

```
📦 <project root>
└ 📜 auto-screen-shotter.exe <binary>
└ 📂 images
 ├ 📂 2024-04-01
 │  ├ 📜 2024-04-01_11-57-53.png
 │  └ 📜 2024-04-01_11-58-53.png
 │  └ 📜 ...
 ├ 📂 ... 
```

## Release
cargo build --release

## Release windows without console
cargo rustc --release -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"

## Dev
cargo run
