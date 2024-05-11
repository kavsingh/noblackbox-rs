use crate::components::{
	button::Button,
	card::{Card, CardContent, CardHeader, CardTitle},
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

type Path = Vec<(i32, i32)>;

#[island]
pub fn Sketchpad() -> impl IntoView {
	let (paths, set_paths) = create_signal(Vec::<Path>::new());

	let on_pointer_down = move |ev: ev::PointerEvent| {
		let x = ev.client_x();
		let y = ev.client_y();

		set_paths.update(|paths| {
			paths.push(vec![(x, y)]);
		});
	};

	let on_undo_click = move |_| {
		set_paths.update(|paths| {
			paths.pop();
		});
	};

	create_effect(move |_| {
		log::info!("paths: {:?}", paths.get());
	});

	view! {
		<div class="flex gap-4">
			<canvas
				on:pointerdown=on_pointer_down
				id="train"
				width="400"
				height="400"
				class="bg-white stroke-black"
			></canvas>
			<Button on:click=on_undo_click>Undo</Button>
		</div>
	}
}
