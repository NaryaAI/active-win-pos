#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi(object)]
pub struct WindowPosition {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

#[napi(object)]
pub struct ActiveWindow {
  pub title: String,
  pub process_path: String,
  pub app_name: String,
  pub window_id: String,
  pub process_id: u32,
  pub position: WindowPosition,
}

impl From<active_win_pos_rs::ActiveWindow> for ActiveWindow {
  fn from(active_window: active_win_pos_rs::ActiveWindow) -> Self {
    ActiveWindow {
      title: active_window.title,
      process_path: active_window.process_path.to_string_lossy().to_string(),
      app_name: active_window.app_name,
      window_id: active_window.window_id,
      process_id: active_window.process_id as u32,
      position: WindowPosition {
        x: active_window.position.x,
        y: active_window.position.y,
        width: active_window.position.width,
        height: active_window.position.height,
      },
    }
  }
}

#[napi]
pub fn get_active_window() -> napi::Result<ActiveWindow> {
  active_win_pos_rs::get_active_window()
    .map(|active_window| ActiveWindow::from(active_window))
    .map_err(|_| {
      napi::Error::new(
        napi::Status::GenericFailure,
        "Failed to get active window".to_string(),
      )
    })
}
