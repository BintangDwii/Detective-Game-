use leptos::prelude::*;

use crate::game::LEVELS;

#[component]
pub fn Briefing(
    level: RwSignal<u8>,
    completed: RwSignal<[bool; 5]>,
    on_insert_hint: Callback<()>,
) -> impl IntoView {
    let badge = move || LEVELS[(level.get() - 1) as usize].badge;
    let title = move || LEVELS[(level.get() - 1) as usize].title;
    let story = move || LEVELS[(level.get() - 1) as usize].story;
    let hint = move || LEVELS[(level.get() - 1) as usize].target_hint;
    let status = move || {
        if completed.get()[(level.get() - 1) as usize] {
            "Status: ✅ Selesai"
        } else {
            "Status: Belum Selesai"
        }
    };

    view! {
        <div class="bg-slate-800 border border-slate-700 rounded-xl p-4 shadow-md flex flex-col justify-between">
            <div>
                <p class="text-xs uppercase tracking-wider text-slate-400 mb-2 font-semibold">"Misi Aktif"</p>
                <div class="flex items-center justify-between mb-2">
                    <span class="bg-blue-500/20 text-blue-400 border border-blue-500/30 text-[10px] font-bold px-2 py-0.5 rounded-full uppercase tracking-wider">
                        {badge}
                    </span>
                    <span class="text-xs text-slate-400 font-mono">{status}</span>
                </div>
                <h2 class="text-base font-bold text-slate-100 mb-1">{title}</h2>
                <p class="text-xs text-slate-200 leading-relaxed mb-3 bg-blue-500/10 border-l-4 border-blue-500 rounded-r-xl p-3" inner_html=story></p>

                <div class="bg-slate-950 p-2.5 rounded-lg border border-slate-700 mb-3">
                    <p class="text-[11px] font-semibold text-blue-400 mb-1">"🎯 Target Query SQL:"</p>
                    <code class="text-[11px] text-emerald-400 font-mono block break-words">{hint}</code>
                </div>
            </div>

            <div class="flex gap-2">
                <button
                    on:click=move |_| on_insert_hint.run(())
                    class="w-full bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs py-1.5 px-2 rounded border border-slate-700 transition flex items-center justify-center gap-1"
                >
                    <span>"📋"</span> " Salin Query Contoh"
                </button>
            </div>
        </div>
    }
}
