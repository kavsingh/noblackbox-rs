use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::error_template::{AppError, ErrorTemplate};
use crate::layouts::app_root::AppRoot;
use crate::pages::evaluate::Evaluate;
use crate::pages::train::Train;

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	view! {
		<Stylesheet id="leptos" href="/pkg/noblackbox-rs.css" />
		<Title text="noblackbox" />
		<Router fallback=|| {
			let mut outside_errors = Errors::default();
			outside_errors.insert_with_default_key(AppError::NotFound);
			view! { <ErrorTemplate outside_errors /> }.into_view()
		}>
			<Routes>
				<Route path="/" view=AppRoot>
					<Route
						path=""
						view=|| {
							view! { <Redirect path="/train" /> }
						}
					/>

					<Route path="train" view=Train />
					<Route path="evaluate" view=Evaluate />
				</Route>
			</Routes>
		</Router>
	}
}
