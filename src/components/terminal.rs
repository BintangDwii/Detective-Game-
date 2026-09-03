use leptos::ev::KeyboardEvent;
use leptos::html::Textarea;
use leptos::prelude::*;

#[component]
pub fn Terminal(
    sql: RwSignal<String>,
    on_run: Callback<()>,
    on_clear: Callback<()>,
    on_snippet: Callback<String>,
) -> impl IntoView {
    let ta: NodeRef<Textarea> = NodeRef::new();

    let snippets = ["SELECT * FROM ", "JOIN ", "WHERE ", "LIKE ", "GROUP BY ", "ORDER BY "];

    view! {
        <div class="bg-slate-800 border border-slate-700 rounded-xl shadow-md overflow-hidden flex flex-col">
            <div class="bg-black/20 px-4 py-3 border-b border-slate-700 flex items-center justify-between">
                <div class="flex items-center space-x-2">
                    <span class="w-3 h-3 bg-red-500 rounded-full inline-block"></span>
                    <span class="w-3 h-3 bg-yellow-500 rounded-full inline-block"></span>
                    <span class="w-3 h-3 bg-green-500 rounded-full inline-block"></span>
                    <span class="text-xs font-mono text-slate-400 ml-2">"sql_terminal_v2.1"</span>
                </div>
                <div class="flex space-x-1">
                    <For each=move || snippets key=|s| *s let:snip>
                        {
                            let label = snip.trim().to_string();
                            let text = snip.to_string();
                            view! {
                                <button
                                    on:click=move |_| on_snippet.run(text.clone())
                                    class="px-2 py-0.5 bg-slate-800 hover:bg-slate-700 text-[10px] font-mono rounded text-slate-300"
                                >
                                    {label}
                                </button>
                            }
                        }
                    </For>
                </div>
            </div>

            <div class="p-3 bg-slate-950">
                <textarea
                    node_ref=ta
                    rows="4"
                    prop:value=move || sql.get()
                    on:input=move |_| {
                        if let Some(el) = ta.get() {
                            sql.set(el.value());
                        }
                    }
                    on:keydown=move |ev: KeyboardEvent| {
                        if ev.ctrl_key() && ev.key() == "Enter" {
                            ev.prevent_default();
                            on_run.run(());
                        }
                    }
                    placeholder="Ketik perintah SQL di sini... (Contoh: SELECT * FROM warga;)"
                    class="w-full bg-slate-950 text-slate-100 font-mono-code text-sm p-3 rounded-lg border border-slate-700 focus:outline-none focus:border-blue-500 transition resize-none leading-relaxed"
                ></textarea>

                <div class="flex items-center justify-between mt-3 pt-3 border-t border-slate-700">
                    <span class="text-[11px] text-slate-500 font-mono">"Tip: Tekan Ctrl + Enter untuk eksekusi"</span>
                    <div class="flex space-x-2">
                        <button
                            on:click=move |_| on_clear.run(())
                            class="px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs rounded-md font-medium transition"
                        >
                            "Bersihkan"
                        </button>
                        <button
                            on:click=move |_| on_run.run(())
                            class="px-5 py-2 bg-blue-500 hover:bg-blue-600 text-white text-sm font-semibold rounded-md shadow transition flex items-center gap-2"
                        >
                            <svg width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
                                <path d="M10.804 8 5 4.633v6.734L10.804 8zm.792-.696a.802.802 0 0 1 0 1.392l-6.363 3.692C4.713 12.69 4 12.345 4 11.692V4.308c0-.653.713-.998 1.233-.696l6.363 3.692z"/>
                            </svg>
                            "Jalankan Query"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
