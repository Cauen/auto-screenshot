![image](https://github.com/Cauen/auto-screenshot/assets/8796757/fd4f9bdf-1101-4bb4-8249-52df0a3f9b44)

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

## Dev
cargo run