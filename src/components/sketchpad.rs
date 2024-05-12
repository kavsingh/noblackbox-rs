use crate::components::button::Button;
use leptos::*;
use wasm_bindgen::JsCast as _;
use web_sys::HtmlCanvasElement;

#[component]
pub fn Sketchpad(#[prop(into)] on_save: Callback<Vec<Path>>) -> impl IntoView {
	let (is_drawing, set_is_drawing) = create_signal(false);
	let (paths, set_paths) = create_signal(Vec::<Path>::new());
	let canvas_ref = create_node_ref::<html::Canvas>();

	let start_draw_path = move |event: ev::PointerEvent| {
		let canvas: HtmlCanvasElement = leptos::event_target(&event);
		let rect = canvas.get_bounding_client_rect();
		let x = event.client_x();
		let y = event.client_y();

		set_is_drawing(true);
		set_paths.update(|paths| {
			paths.push(vec![(x - rect.left() as i32, y - rect.top() as i32)]);
		});
	};

	let draw_path = move |event: ev::PointerEvent| {
		if !is_drawing() {
			return;
		}

		let canvas: HtmlCanvasElement = leptos::event_target(&event);
		let rect = canvas.get_bounding_client_rect();
		let x = event.client_x();
		let y = event.client_y();

		set_paths.update(|paths| {
			if let Some(path) = paths.last_mut() {
				path.push((x - rect.left() as i32, y - rect.top() as i32));
			}
		});
	};

	let end_draw_path = move |_| {
		set_is_drawing(false);
	};

	let on_undo_click = move |_| {
		set_paths.update(|paths| {
			paths.pop();
		});
	};

	let on_save_click = move |_| {
		on_save(paths());
		set_paths(vec![]);
	};

	create_effect(move |_| {
		let current_paths = paths();

		if let Some(canvas) = canvas_ref() {
			let ctx = canvas
				.get_context("2d")
				.unwrap()
				.unwrap()
				.dyn_into::<web_sys::CanvasRenderingContext2d>()
				.unwrap();

			ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
			ctx.begin_path();
			ctx.set_line_width(2.0);
			ctx.set_stroke_style(&"black".into());

			for path in current_paths {
				if path.is_empty() {
					continue;
				}

				ctx.move_to(path[0].0 as f64, path[0].1 as f64);

				for point in path.iter().skip(1) {
					ctx.line_to(point.0 as f64, point.1 as f64);
				}
			}

			ctx.stroke();
		}
	});

	view! {
		<div class="flex gap-4">
			<div class="rounded-md overflow-hidden">
				<canvas
					width="400"
					height="400"
					class="bg-white stroke-black"
					node_ref=canvas_ref
					on:pointerdown=start_draw_path
					on:pointermove=draw_path
					on:pointerup=end_draw_path
					on:pointerleave=end_draw_path
				></canvas>
			</div>
			<div class="flex flex-col gap-6">
				<Button on:click=on_save_click>Save</Button>
				<Button on:click=on_undo_click>Undo</Button>
			</div>
		</div>
	}
}

type Path = Vec<(i32, i32)>;
