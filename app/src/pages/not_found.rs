use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="max-w-2xl min-w-0 flex-auto px-4 py-16 lg:max-w-none lg:pr-0 lg:pl-8 xl:px-16">
        <div class="flex h-full flex-col items-center justify-center text-center">
          <p class="font-display text-sm font-medium text-slate-900 dark:text-white">
            404
          </p>
          <h1 class="mt-3 font-display text-3xl tracking-tight text-slate-900 dark:text-white">
            Page not found
          </h1>
          <p class="mt-2 text-sm text-slate-500 dark:text-slate-400">
            "Sorry, we couldn't find the page you're looking for."
          </p>
          <a
            href="/"
            class="mt-8 text-sm font-medium text-slate-900 dark:text-white"
          >
            Go back home
          </a>
        </div>
      </div>
    }
}
