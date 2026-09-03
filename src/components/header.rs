use leptos::prelude::*;

const LEVEL_NAMES: [&str; 5] = [
    "Misi 1: SELECT",
    "Misi 2: JOIN",
    "Misi 3: LIKE",
    "Misi 4: GROUP BY",
    "Misi 5: TUDUH",
];

#[component]
pub fn Header(
    level: RwSignal<u8>,
    on_accuse: Callback<()>,
    on_reset: Callback<()>,
) -> impl IntoView {
    view! {
        <header class="bg-slate-800 border-b border-slate-700 sticky top-0 z-30 px-4 py-3 shadow-lg">
            <div class="max-w-7xl mx-auto flex flex-col md:flex-row items-center justify-between gap-3">
                <div class="flex items-center space-x-3">
                    <div>
                        <h1 class="font-bold text-lg md:text-xl tracking-tight text-slate-100 flex items-center gap-2.5">
                            <span class="text-blue-500">"Mata Elang"</span> " Database Kepolisian"
                        </h1>
                        <p class="text-xs text-slate-400">"Kasus Pencurian Lukisan \"Monnalisa KW Super\""</p>
                    </div>
                </div>

                <div class="flex items-center space-x-1 md:space-x-2 bg-slate-950 p-1.5 rounded-lg border border-slate-700 text-xs">
                    <For
                        each=move || [1u8, 2, 3, 4, 5]
                        key=|l| *l
                        let:lvl
                    >
                        {
                            let name = LEVEL_NAMES[(lvl - 1) as usize];
                            let sep = lvl < 5;
                            view! {
                                <button
                                    on:click=move |_| level.set(lvl)
                                    class=move || {
                                        if level.get() == lvl {
                                            "px-3 py-1 rounded font-semibold transition-all bg-blue-500 text-white"
                                        } else {
                                            "px-3 py-1 rounded font-semibold transition-all text-slate-400 hover:text-slate-200"
                                        }
                                    }
                                >
                                    {name}
                                </button>
                                <Show when=move || sep fallback=|| ()>
                                    <span class="text-slate-600">"➔"</span>
                                </Show>
                            }
                        }
                    </For>
                </div>

                <div class="flex items-center space-x-2">
                    <span class="bg-emerald-500/20 text-emerald-500 border border-emerald-500 px-3 py-1 rounded-full text-xs font-semibold">
                        "Koneksi Aman"
                    </span>
                    <button
                        on:click=move |_| on_accuse.run(())
                        class="bg-red-600 hover:bg-red-500 text-white font-bold px-4 py-2 rounded-lg text-xs md:text-sm shadow-md transition-all flex items-center gap-1.5 animate-pulse"
                    >
                        <span>"⚖️"</span> " TUDUH PELAKU"
                    </button>
                    <button
                        on:click=move |_| on_reset.run(())
                        class="bg-slate-800 hover:bg-slate-700 text-slate-300 px-3 py-2 rounded-lg text-xs border border-slate-700 transition-all"
                    >
                        "🔄 Reset"
                    </button>
                </div>
            </div>
        </header>
    }
}
