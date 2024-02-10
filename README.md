# rust-guassian-blur
Simple Guassian blur implemented in Rust.

## Usage
Use the following where Vec<u8> is an array of RGBA values. See `main.rs` for full demo.
```rs
blur(w: i32, h: i32, pixels: &mut Vec<u8>, radius: f32) -> Vec<u8>
```
