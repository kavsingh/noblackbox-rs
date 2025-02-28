// https://github.com/shadcn-ui/ui/blob/main/apps/v4/registry/new-york-v4/ui/button.tsx
// TODO: variants, somehow. maybe https://github.com/gaucho-labs/tailwind-fuse?

use leptos::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
	view! {
		<button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-[color,box-shadow] disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive \
		border border-input bg-background shadow-xs hover:bg-accent hover:text-accent-foreground \
		h-9 px-4 py-2 has-[>svg]:px-3">{children()}</button>
	}
}
