use leptos::prelude::*;

use crate::game::{pin_locked_title, pin_unlocked_title, LEVELS};

#[component]
fn PinItem(
    lvl: u8,
    completed: RwSignal<[bool; 5]>,
) -> impl IntoView {
    let unlocked = Memo::new(move |_| completed.get()[(lvl - 1) as usize]);
    let status_text = move || if unlocked.get() { "✅ Terbuka" } else { "🔒 Terkunci" };
    let status_class = move || {
        if unlocked.get() {
            "text-[10px] text-emerald-400 font-bold"
        } else {
            "text-[10px] text-slate-500"
        }
    };
    let card_class = move || {
        if unlocked.get() {
            "bg-slate-800 border border-blue-500/50 rounded-lg p-3 transition"
        } else {
            "bg-slate-950 border border-slate-800/80 rounded-lg p-3 opacity-50 transition"
        }
    };
    let title = move || {
        if unlocked.get() {
            LEVELS[(lvl - 1) as usize].pin_title.to_string()
        } else {
            pin_locked_title(lvl).to_string()
        }
    };
    let body = move || {
        if unlocked.get() {
            LEVELS[(lvl - 1) as usize].pin_text.to_string()
        } else {
            LEVELS[(lvl - 1) as usize].pin_locked_text.to_string()
        }
    };

    view! {
        <div class=card_class>
            <div class="flex items-center justify-between mb-1">
                <span class="text-[10px] font-bold text-blue-400 uppercase">
                    {move || pin_unlocked_title(lvl)}
                </span>
                <span class=status_class>{status_text}</span>
            </div>
            <p class="text-xs font-semibold text-slate-200 mb-2">{title}</p>
            <div class="text-[11px] text-slate-400 italic">{body}</div>
        </div>
    }
}

#[component]
pub fn Pinboard(
    completed: RwSignal<[bool; 5]>,
    on_accuse: Callback<()>,
) -> impl IntoView {
    let count = move || {
        let n = completed.get().iter().take(4).filter(|b| **b).count();
        format!("{n}/4 Terbuka")
    };

    view! {
        <div class="bg-slate-800 border border-slate-700 rounded-xl p-4 shadow-md flex-1 flex flex-col">
            <div class="border-b border-slate-700 pb-2 mb-3 flex items-center justify-between">
                <h3 class="text-sm font-bold text-blue-400 flex items-center gap-1.5">
                    <span>"📌"</span> " PAPAN BUKTI TERKUMPUL"
                </h3>
                <span class="text-[10px] bg-slate-800 text-slate-300 px-2 py-0.5 rounded">{count}</span>
            </div>

            <p class="text-[11px] text-slate-400 mb-3">
                "Bukti dan profil kunci akan otomatis tersemat di sini setelah Anda menemukan query yang tepat pada setiap Misi."
            </p>

            <div class="space-y-3 overflow-y-auto flex-1 max-h-[580px] pr-1">
                <PinItem lvl=1 completed=completed />
                <PinItem lvl=2 completed=completed />
                <PinItem lvl=3 completed=completed />
                <PinItem lvl=4 completed=completed />
            </div>

            <div class="mt-4 pt-3 border-t border-slate-700">
                <button
                    on:click=move |_| on_accuse.run(())
                    class="w-full bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-3 rounded-lg text-xs shadow transition flex items-center justify-center gap-1.5"
                >
                    <span>"⚖️"</span> " TANGKAP & TUDUH TERSANGKA"
                </button>
            </div>
        </div>
    }
}
