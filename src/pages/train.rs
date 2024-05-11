use leptos::*;

use crate::components::card::{Card, CardContent, CardHeader, CardTitle};

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
				<canvas id="train" width="400" height="400"></canvas>
			</CardContent>
		</Card>
	}
}
