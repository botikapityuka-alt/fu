// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::os::raw::c_int;
#[link(name = "fractal_engine", kind = "static")]
extern "C" {
    fn generate_fractal(
        buffer: *mut u8,
        width: std::os::raw::c_int,
        height: std::os::raw::c_int,
        zoom: f64,
        offset_x: f64,
        offset_y: f64,
    );
}

#[tauri::command]
fn get_fractal_image(width: i32, height: i32, zoom: f64, x: f64, y: f64) -> Vec<u8> {
    let buffer_size = (width * height * 4) as usize; // RGBA csatornák
    let mut buffer = vec![0u8; buffer_size];

    // Unsafe blokk a C/C++ függvény hívásához
    unsafe {
        generate_fractal(
            buffer.as_mut_ptr(),
            width,
            height,
            zoom,
            x,
            y,
        );
    }

    buffer
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_fractal_image])
        .run(tauri::generate_context!())
        .expect("Hiba a Tauri alkalmazás futtatásakor");
}