// https://ui.shadcn.com/docs/components/button

use leptos::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
	view! {
		<button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground">
			{children()}
		</button>
	}
}
