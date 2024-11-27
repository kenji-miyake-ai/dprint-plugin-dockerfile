use std::env;
use std::path::Path;

use dprint_plugin_dockerfile::configuration::resolve_config;
use dprint_plugin_dockerfile::format_text;

fn main() {
  // Call wasm_plugin.rs's format. Get file_path from arg.
  let args: Vec<String> = env::args().collect();
  let file_path = Path::new(&args[1]);
  let file_bytes = std::fs::read(file_path).unwrap();
  let file_text = String::from_utf8(file_bytes).unwrap();
  let config = resolve_config(Default::default(), &Default::default()).config;

  let result = format_text(file_path, &file_text, &config).unwrap();
  match result {
    Some(formatted_text) => {
      std::fs::write(file_path, formatted_text).unwrap();
    }
    None => {}
  }
}
