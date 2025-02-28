// https://github.com/shadcn-ui/ui/blob/main/apps/v4/registry/new-york-v4/ui/card.tsx

use leptos::*;

#[component]
pub fn Card(#[prop(optional)] class: Option<&'static str>, children: Children) -> impl IntoView {
	view! {
		<div class=format!(
			"bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm {}",
			class.unwrap_or_default(),
		)>{children()}</div>
	}
}

#[component]
pub fn CardHeader(
	#[prop(optional)] class: Option<&'static str>,
	children: Children,
) -> impl IntoView {
	view! {
		<div class=format!(
			"flex flex-col gap-1.5 px-6 {}",
			class.unwrap_or_default(),
		)>{children()}</div>
	}
}

#[component]
pub fn CardTitle(
	#[prop(optional)] class: Option<&'static str>,
	children: Children,
) -> impl IntoView {
	view! {
		<div class=format!(
			"leading-none font-semibold {}",
			class.unwrap_or_default(),
		)>{children()}</div>
	}
}

#[component]
pub fn CardDescription(
	#[prop(optional)] class: Option<&'static str>,
	children: Children,
) -> impl IntoView {
	view! {
		<div class=format!(
			"text-muted-foreground text-sm {}",
			class.unwrap_or_default(),
		)>{children()}</div>
	}
}

#[component]
pub fn CardContent(
	#[prop(optional)] class: Option<&'static str>,
	children: Children,
) -> impl IntoView {
	view! { <div class=format!("px-6 {}", class.unwrap_or_default())>{children()}</div> }
}

#[component]
pub fn CardFooter(
	#[prop(optional)] class: Option<&'static str>,
	children: Children,
) -> impl IntoView {
	view! {
		<div class=format!(
			"flex items-center px-6 {}",
			class.unwrap_or_default(),
		)>{children()}</div>
	}
}
