use leptos::html::Canvas;
use leptos::prelude::*;

use crate::canvas::{draw_avatar, draw_chart};
use crate::models::{chart_data, derive_evidence, EvidenceItem, Feedback, FeedbackKind, QueryResult, Tab};

fn tab_class(active: bool, base: &str) -> String {
    if active {
        format!("px-3 py-1 bg-blue-500 text-white rounded text-xs font-bold transition {base}")
    } else {
        format!("px-3 py-1 bg-slate-800 text-slate-300 hover:text-white rounded text-xs font-bold transition {base}")
    }
}

#[component]
fn EvidenceCard(item: EvidenceItem) -> impl IntoView {
    let nr: NodeRef<Canvas> = NodeRef::new();
    let key = item.canvas_key.clone();
    let title = item.title.clone();
    Effect::new(move |_| {
        if let Some(c) = nr.get() {
            draw_avatar(&c, &key, &title);
        }
    });

    view! {
        <div class="bg-slate-950 border border-dashed border-slate-700 rounded-lg p-3 flex items-center space-x-3">
            <canvas
                node_ref=nr
                width="70"
                height="70"
                class="rounded border border-slate-700 bg-slate-900"
            ></canvas>
            <div class="text-xs space-y-1">
                <p class="font-bold text-blue-400">{item.title.clone()}</p>
                <p class="text-slate-300 text-[11px]">{item.subtitle.clone()}</p>
                <p class="text-slate-500 text-[10px] font-mono">{item.detail.clone()}</p>
            </div>
        </div>
    }
}

#[component]
fn ChartView(result: RwSignal<Option<QueryResult>>, seq: RwSignal<u64>) -> impl IntoView {
    let nr: NodeRef<Canvas> = NodeRef::new();
    Effect::new(move |_| {
        let _s = seq.get();
        let data = result.get().map(|r| chart_data(&r)).unwrap_or_default();
        if let Some(c) = nr.get() {
            draw_chart(&c, &data);
        }
    });

    view! {
        <div>
            <p class="text-xs text-slate-400 mb-2">"Grafik visualisasi ringkasan berdasarkan hasil query:"</p>
            <div class="chart-container">
                <canvas
                    node_ref=nr
                    width="600"
                    height="260"
                    class="w-full h-full"
                ></canvas>
            </div>
        </div>
    }
}

#[component]
pub fn Results(
    tab: RwSignal<Tab>,
    feedback: RwSignal<Option<Feedback>>,
    result: RwSignal<Option<QueryResult>>,
    seq: RwSignal<u64>,
) -> impl IntoView {
    let feedback_class = move || match feedback.get().as_ref().map(|f| &f.kind) {
        Some(FeedbackKind::Error) => "mb-3 p-3 rounded-lg text-xs font-medium border bg-red-950/80 text-red-300 border-red-800",
        Some(FeedbackKind::Success) => "mb-3 p-3 rounded-lg text-xs font-medium border bg-emerald-950/80 text-emerald-300 border-emerald-800",
        _ => "mb-3 p-3 rounded-lg text-xs font-medium border bg-slate-700/60 text-slate-200 border-slate-600",
    };
    let feedback_text = move || {
        feedback
            .get()
            .map(|f| f.message)
            .unwrap_or_default()
    };

    let meta = move || match result.get() {
        Some(r) => format!("{} baris ditemukan", r.row_count()),
        None => "0 baris ditemukan".to_string(),
    };
    let evidence_count = move || match result.get() {
        Some(r) => derive_evidence(&r, seq.get_untracked()).len().to_string(),
        None => "0".to_string(),
    };

    let labels = move || result.get().map(|r| r.labels).unwrap_or_default();
    let body = move || {
        result
            .get()
            .map(|r| {
                r.rows
                    .into_iter()
                    .map(|row| {
                        let cells = row
                            .into_iter()
                            .map(|c| {
                                view! {
                                    <td class="p-3 font-normal whitespace-nowrap">{c}</td>
                                }
                            })
                            .collect::<Vec<_>>();
                        view! {
                            <tr class="border-b border-slate-700 text-slate-200 hover:bg-white/[0.02]">
                                {cells}
                            </tr>
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    };
    let has_result = move || result.get().is_some();
    let evidence = move || match result.get() {
        Some(r) => derive_evidence(&r, seq.get()),
        None => Vec::new(),
    };

    view! {
        <div class="bg-slate-800 border border-slate-700 rounded-xl shadow-md flex-1 flex flex-col overflow-hidden">
            <div class="bg-black/20 px-4 py-2 border-b border-slate-700 flex items-center justify-between">
                <div class="flex space-x-2">
                    <button
                        on:click=move |_| tab.set(Tab::Table)
                        class=move || tab_class(tab.get() == Tab::Table, "")
                    >
                        "📊 Tabel Data"
                    </button>
                    <button
                        on:click=move |_| tab.set(Tab::Visual)
                        class=move || tab_class(tab.get() == Tab::Visual, "flex items-center gap-1")
                    >
                        <span>"🖼️"</span>
                        " Galeri Bukti "
                        <span class="bg-blue-500/20 text-blue-400 text-[9px] px-1.5 rounded-full">
                            {evidence_count}
                        </span>
                    </button>
                    <button
                        on:click=move |_| tab.set(Tab::Chart)
                        class=move || tab_class(tab.get() == Tab::Chart, "")
                    >
                        "📈 Visualisasi Analytics"
                    </button>
                </div>
                <span class="text-[11px] font-mono text-slate-400">{meta}</span>
            </div>

            <div class="p-4 flex-1 overflow-y-auto max-h-[420px]">
                <Show when=move || feedback.get().is_some() fallback=|| ()>
                    <div class=feedback_class>{feedback_text}</div>
                </Show>

                <Show
                    when=move || tab.get() == Tab::Table
                    fallback=|| ()
                >
                    <div class="block overflow-x-auto">
                        <Show
                            when=has_result
                            fallback=move || view! {
                                <div class="text-center py-12 text-slate-500">
                                    <span class="text-3xl block mb-2">"⌨️"</span>
                                    <p class="text-xs">"Ketik SQL query Anda di atas dan klik \"JALANKAN QUERY\""</p>
                                </div>
                            }
                        >
                            <table class="w-full text-left text-xs font-mono border-collapse">
                                <thead>
                                    <tr class="bg-slate-950 text-slate-400 border-b border-slate-700">
                                        <For each=labels key=|l| l.clone() let:label>
                                            <th class="p-3 font-semibold whitespace-nowrap">{label}</th>
                                        </For>
                                    </tr>
                                </thead>
                                <tbody>{body}</tbody>
                            </table>
                        </Show>
                    </div>
                </Show>

                <Show when=move || tab.get() == Tab::Visual fallback=|| ()>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                        <Show
                            when=move || !evidence().is_empty()
                            fallback=move || view! {
                                <div class="col-span-2 text-center py-8 text-slate-500 text-xs">
                                    "Tidak ada barang bukti visual (foto_profil/foto_mobil) dalam hasil query ini."
                                </div>
                            }
                        >
                            <For each=evidence key=|item| item.key.clone() let:item>
                                <EvidenceCard item=item />
                            </For>
                        </Show>
                    </div>
                </Show>

                <Show when=move || tab.get() == Tab::Chart fallback=|| ()>
                    <ChartView result=result seq=seq />
                </Show>
            </div>
        </div>
    }
}
