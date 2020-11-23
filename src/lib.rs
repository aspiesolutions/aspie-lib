#[cfg(target_arch = "wasm32")]
extern crate js_sys;
#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
extern crate web_sys;
#[cfg(target_arch = "wasm32")]
pub mod wasm;
#[cfg(target_arch = "wasm32")]
use wasm::*;
// desktop imports
#[cfg(not(target_arch = "wasm32"))]
extern crate reqwest;
#[cfg(not(target_arch = "wasm32"))]
extern crate tokio;

#[cfg(not(target_arch = "wasm32"))]
pub mod desktop;
#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_imports)]
use desktop::*;
