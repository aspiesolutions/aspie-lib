// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
pub mod utils;
use wasm_bindgen::prelude::*;

#[cfg(feature = "aspie")]
pub mod aspie;
#[cfg(feature = "auth0")]
pub mod auth0;
#[cfg(feature = "cloudflare")]
pub mod cloudflare;
#[cfg(feature = "digitalocean")]
pub mod digitalocean;
#[cfg(feature = "github")]
pub mod github;
#[cfg(feature = "godaddy")]
pub mod godaddy;
#[cfg(feature = "heroku")]
pub mod heroku;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
  alert("Hello, aspie-wasm-lib!");
}
