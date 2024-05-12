use crate::components::{
	card::{Card, CardContent, CardHeader, CardTitle},
	sketchpad::Sketchpad,
};
use leptos::*;

#[component]
pub fn Train() -> impl IntoView {
	view! {
		<Card>
			<CardHeader>
				<CardTitle>
					<h1>train</h1>
				</CardTitle>
			</CardHeader>
			<CardContent>
				<Sketchpad/>
			</CardContent>
		</Card>
	}
}
