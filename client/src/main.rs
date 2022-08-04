mod app;

use app::App;
use wasm_bindgen_console_logger::DEFAULT_LOGGER;

fn main() {
  log::set_logger(&DEFAULT_LOGGER).unwrap();
  log::set_max_level(log::LevelFilter::Info);

  yew::start_app::<App>();
}
