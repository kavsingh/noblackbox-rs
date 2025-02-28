use std::collections::HashMap;

use leptos::*;

use crate::components::button::Button;
use crate::components::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::sketchpad::Sketchpad;

type Paths = Vec<Vec<(i32, i32)>>;

const TRAINING_LABELS: [&str; 8] = [
	"car", "fish", "house", "tree", "bicycle", "guitar", "pencil", "clock",
];

#[derive(serde::Serialize, Debug, Clone)]
struct TrainingData {
	session: i32,
	student: &'static str,
	drawings: HashMap<&'static str, Paths>,
}

#[component]
pub fn Train() -> impl IntoView {
	view! {
		<Card>
			<CardHeader>
				<CardTitle>
					<span class="flex gap-2">
						<h2>train</h2>
					</span>
				</CardTitle>
			</CardHeader>
			<CardContent>
				<Trainer />
			</CardContent>
		</Card>
	}
}

#[island]
fn Trainer() -> impl IntoView {
	let (session, set_session) = create_signal(TrainingData {
		session: 123456,
		student: "__leptos__",
		drawings: HashMap::<&str, Paths>::new(),
	});

	let current_label = move || {
		TRAINING_LABELS
			.iter()
			.find(|l| !session().drawings.contains_key(*l))
			.copied()
	};

	let save_drawing = move |paths: Paths| {
		if let Some(label) = current_label() {
			set_session.update(|current| {
				current.drawings.insert(label, paths);
			});
		};
	};

	let save_session = move |_| {
		log::debug!("save session {:?}", session());
	};

	view! {
		<Show when=move || current_label().is_some()>
			<h3 class="mb-2 font-normal text-muted-foreground">
				draw a
				<span class="font-semibold text-foreground">
					{move || current_label().unwrap_or("--")}
				</span>
			</h3>
			<Sketchpad on_save=save_drawing />
		</Show>
		<Show when=move || {
			current_label().is_none()
		}>done! <Button on:click=save_session>save training data</Button></Show>
	}
}
