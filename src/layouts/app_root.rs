use leptos::*;
use leptos_router::*;

use crate::components::card::Card;

#[component]
pub fn AppRoot() -> impl IntoView {
	view! {
		<header class="p-4">
			<Card>
				<div class="p-2 flex gap-8">
					<h1 class="font-semibold">
						<A href="/">noblackbox</A>
					</h1>
					<nav class="flex gap-4">
						<A href="/train">train</A>
						<A href="/evaluate">evaluate</A>
					</nav>
				</div>
			</Card>
		</header>
		<main class="p-4">
			<Outlet/>
		</main>
	}
}
