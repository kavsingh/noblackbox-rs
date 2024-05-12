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

	let current_label = create_memo::<Option<&str>>(move |_| {
		if let Some(drawing) = drawings().iter().find(|&(_, d)| d.is_none()) {
			Some(drawing.0)
		} else {
			None
		}
	});

	let save_drawing = move |drawing: Vec<Vec<(i32, i32)>>| {
		let label = match current_label() {
			Some(l) => l,
			None => return,
		};

		set_drawings.update(|current| {
			let label_index = current.iter().position(|&(l, _)| l == label);

			if let Some(index) = label_index {
				current[index].1 = Some(drawing);
			}
		})
	};

	view! {
		<h3 class="mb-2 font-normal text-muted-foreground">
			draw a
			<span class="font-semibold text-foreground">
				{move || current_label().unwrap_or("--")}
			</span>
		</h3>
		<Sketchpad on_save=save_drawing/>
	}
}

type Drawing = (&'static str, Option<Vec<Vec<(i32, i32)>>>);
