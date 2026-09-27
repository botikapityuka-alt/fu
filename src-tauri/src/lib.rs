// Ez a függvény fogad egy üzenetet a React-tól, és válaszol rá
#[tauri::command]
fn teszt_uzenet(szoveg: String) -> String {
    format!("Rust motor üzenete: Megkaptam ezt: '{}' -> Slayyy! 🔥", szoveg.to_uppercase())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Itt regisztráljuk a függvényt, hogy a React elérhesse
        .invoke_handler(tauri::generate_handler![teszt_uzenet])
        .run(tauri::generate_context!())
        .expect("Hiba történt a Tauri alkalmazás futtatása közben");
}
