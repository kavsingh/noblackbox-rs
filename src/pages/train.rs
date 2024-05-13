use std::collections::HashMap;

use crate::components::{
	button::Button,
	card::{Card, CardContent, CardHeader, CardTitle},
	sketchpad::Sketchpad,
};
use leptos::*;

type Paths = Vec<Vec<(i32, i32)>>;
type Drawing = (&'static str, Option<Paths>);

#[derive(serde::Serialize, Debug)]
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
				<Trainer/>
			</CardContent>
		</Card>
	}
}

#[island]
fn Trainer() -> impl IntoView {
	let (drawings, set_drawings) = create_signal::<Vec<Drawing>>(vec![
		("car", None),
		("fish", None),
		("house", None),
		("tree", None),
		("bicycle", None),
		("guitar", None),
		("pencil", None),
		("clock", None),
	]);

	let current_label = move || drawings().iter().find(|d| d.1.is_none()).map(|d| d.0);

	let save_drawing = move |paths: Paths| {
		let label = match current_label() {
			Some(l) => l,
			None => return,
		};

		set_drawings.update(|current| {
			if let Some(index) = current.iter().position(|d| d.0 == label) {
				current[index].1 = Some(paths);
			}
		})
	};

	let save_session = move |_| {
		let mut drawing_map = HashMap::<&str, Paths>::new();

		drawings.get_untracked().iter().for_each(|drawing| {
			if let Some(paths) = &drawing.1 {
				drawing_map.insert(drawing.0, paths.clone());
			}
		});

		let data = TrainingData {
			session: 123456,
			student: "__leptos__",
			drawings: drawing_map,
		};

		log::debug!("save session {:?}", data)
	};

	view! {
		<Show when=move || current_label().is_some()>
			<h3 class="mb-2 font-normal text-muted-foreground">
				draw a
				<span class="font-semibold text-foreground">
					{move || current_label().unwrap_or("--")}
				</span>
			</h3>
			<Sketchpad on_save=save_drawing/>
		</Show>
		<Show when=move || {
			current_label().is_none()
		}>done! <Button on:click=save_session>save training data</Button></Show>
	}
}
