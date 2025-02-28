// https://github.com/shadcn-ui/ui/blob/main/apps/v4/registry/new-york-v4/ui/card.tsx

use leptos::*;

#[component]
pub fn Card(children: Children) -> impl IntoView {
	view! {
		<div class="bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm">
			{children()}
		</div>
	}
}

#[component]
pub fn CardHeader(children: Children) -> impl IntoView {
	view! { <div class="flex flex-col gap-1.5 px-6">{children()}</div> }
}

#[component]
pub fn CardTitle(children: Children) -> impl IntoView {
	view! { <div class="leading-none font-semibold">{children()}</div> }
}

#[component]
pub fn CardDescription(children: Children) -> impl IntoView {
	view! { <div class="text-muted-foreground text-sm">{children()}</div> }
}

#[component]
pub fn CardContent(children: Children) -> impl IntoView {
	view! { <div class="px-6">{children()}</div> }
}

#[component]
pub fn CardFooter(children: Children) -> impl IntoView {
	view! { <div class="flex items-center px-6">{children()}</div> }
}
