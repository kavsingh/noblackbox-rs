// https://ui.shadcn.com/docs/components/card

use leptos::*;

#[component]
pub fn Card(children: Children) -> impl IntoView {
	view! {
		<div class="rounded-xl border border-border bg-card text-card-foreground shadow">
			{children()}
		</div>
	}
}

#[component]
pub fn CardHeader(children: Children) -> impl IntoView {
	view! { <div class="flex flex-col space-y-1.5 p-6">{children()}</div> }
}

#[component]
pub fn CardTitle(children: Children) -> impl IntoView {
	view! { <div class="font-semibold leading-none tracking-tight">{children()}</div> }
}

#[component]
pub fn CardDescription(children: Children) -> impl IntoView {
	view! { <div class="text-sm text-muted-foreground">{children()}</div> }
}

#[component]
pub fn CardContent(children: Children) -> impl IntoView {
	view! { <div class="p-6 pt-0">{children()}</div> }
}

#[component]
pub fn CardFooter(children: Children) -> impl IntoView {
	view! { <div class="flex items-center p-6 pt-0">{children()}</div> }
}
