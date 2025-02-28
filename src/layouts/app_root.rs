use leptos::*;
use leptos_router::*;

use crate::components::card::Card;

#[component]
pub fn AppRoot() -> impl IntoView {
	view! {
		<header class="p-4">
			<Card class="!py-3 !flex-row !gap-8 px-5">
				<h1 class="font-semibold">
					<A href="/">noblackbox</A>
				</h1>
				<nav class="flex gap-4">
					<A href="/train">train</A>
					<A href="/evaluate">evaluate</A>
				</nav>
			</Card>
		</header>
		<main class="p-4">
			<Outlet />
		</main>
	}
}
