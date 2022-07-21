#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn custom(run_message:String){
  println!("I was invoked {}",run_message)
}
#[tauri::command]
fn my_full_send_command()-> String{
  "holafukers".into()
}
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![custom,my_full_send_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
