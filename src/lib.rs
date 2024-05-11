pub mod app;
pub mod components;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod layouts;
pub mod pages;

use log::Level;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	#[allow(unused_imports)]
	use app::*;

	console_log::init_with_level(Level::Trace).expect("error initializing log");
	console_error_panic_hook::set_once();
	leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
