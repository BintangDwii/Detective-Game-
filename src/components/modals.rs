use leptos::html::{Canvas, Input, Select};
use leptos::prelude::*;

use crate::canvas::draw_avatar;
use crate::game::WARGA_OPTIONS;

#[component]
pub fn AccuseModal(
    show: RwSignal<bool>,
    on_submit: Callback<(Option<i32>, String)>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let sel: NodeRef<Select> = NodeRef::new();
    let plat: NodeRef<Input> = NodeRef::new();

    let options = WARGA_OPTIONS
        .into_iter()
        .map(|(id, nama, kerja)| {
            view! {
                <option value=id.to_string()>
                    {format!("ID {id} - {nama} ({kerja})")}
                </option>
            }
        })
        .collect::<Vec<_>>();

    let submit = move |_| {
        let id = sel.get().and_then(|el| {
            let v = el.value();
            if v.is_empty() {
                None
            } else {
                v.parse::<i32>().ok()
            }
        });
        let p = plat.get().map(|el| el.value()).unwrap_or_default();
        on_submit.run((id, p));
    };

    view! {
        <Show when=move || show.get() fallback=|| ()>
            <div class="fixed inset-0 bg-slate-950/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
                <div class="bg-slate-800 border border-blue-500/40 rounded-xl max-w-md w-full p-5 shadow-2xl relative">
                    <button
                        on:click=move |_| on_cancel.run(())
                        class="absolute top-3 right-3 text-slate-400 hover:text-white text-lg"
                    >
                        "✕"
                    </button>

                    <div class="text-center mb-4">
                        <span class="text-4xl block mb-1">"⚖️"</span>
                        <h3 class="text-lg font-bold text-blue-400">"Pengadilan & Penangkapan"</h3>
                        <p class="text-xs text-slate-300">
                            "Masukkan ID Warga tersangka utama berdasarkan investigasi SQL Anda."
                        </p>
                    </div>

                    <div class="space-y-4">
                        <div>
                            <label class="block text-xs font-semibold text-slate-300 mb-1">
                                "Pilih / Masukkan ID Tersangka Utamamu:"
                            </label>
                            <select
                                node_ref=sel
                                class="w-full bg-slate-950 border border-slate-700 text-slate-100 text-sm p-2.5 rounded-lg focus:outline-none focus:border-blue-500 font-mono"
                            >
                                <option value="">"-- Pilih Tersangka berdasarkan ID --"</option>
                                {options.clone()}
                            </select>
                        </div>

                        <div>
                            <label class="block text-xs font-semibold text-slate-300 mb-1">
                                "Nomor Plat Mobil Pelarian:"
                            </label>
                            <input
                                node_ref=plat
                                type="text"
                                placeholder="Contoh: B 1299 DET"
                                class="w-full bg-slate-950 border border-slate-700 text-slate-100 text-sm p-2.5 rounded-lg focus:outline-none focus:border-blue-500 font-mono uppercase"
                            />
                        </div>

                        <div class="bg-slate-950 p-3 rounded-lg border border-slate-700 text-[11px] text-slate-400 space-y-1">
                            <p>
                                "⚠️ " <strong>"Peringatan:"</strong>
                                " Jika tuduhan salah, barang bukti tidak cukup dan tersangka akan melarikan diri ke luar negeri!"
                            </p>
                        </div>

                        <div class="flex space-x-2 pt-2">
                            <button
                                on:click=move |_| on_cancel.run(())
                                class="w-1/2 bg-slate-800 hover:bg-slate-700 text-slate-300 py-2 rounded-lg text-xs font-semibold transition"
                            >
                                "Batal"
                            </button>
                            <button
                                on:click=submit
                                class="w-1/2 bg-red-600 hover:bg-red-500 text-white py-2 rounded-lg text-xs font-bold shadow transition"
                            >
                                "TUDUH & TANGKAP"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[component]
fn CulpritCard() -> impl IntoView {
    let nr: NodeRef<Canvas> = NodeRef::new();
    Effect::new(move |_| {
        if let Some(c) = nr.get() {
            draw_avatar(&c, "pria_jenggot", "Eko Prasetyo");
        }
    });

    view! {
        <div class="bg-slate-950 border border-blue-500/30 p-3 rounded-xl flex items-center justify-center gap-4">
            <canvas
                node_ref=nr
                width="100"
                height="100"
                class="rounded-lg bg-slate-900 border border-slate-700"
            ></canvas>
            <div class="text-left text-xs space-y-1">
                <p class="font-bold text-blue-400 text-sm">"Eko Prasetyo"</p>
                <p class="text-slate-300">"Pekerjaan: Tukang Ledeng (Pencuri KW)"</p>
                <p class="text-slate-300">"Kendaraan: GranMax Merah (B 1299 DET)"</p>
                <p class="text-emerald-400 font-semibold">"Status: Ditangkap & Lukisan Diamankan!"</p>
            </div>
        </div>
    }
}

#[component]
pub fn EndgameModal(
    verdict: RwSignal<Option<bool>>,
    on_close: Callback<()>,
    on_replay: Callback<()>,
) -> impl IntoView {
    view! {
        <Show when=move || verdict.get().is_some() fallback=|| ()>
            <div class="fixed inset-0 bg-slate-950/90 backdrop-blur-md z-50 flex items-center justify-center p-4">
                <div class="bg-slate-800 border-2 rounded-2xl max-w-lg w-full p-6 shadow-2xl text-center relative overflow-hidden">
                    <Show
                        when=move || verdict.get() == Some(true)
                        fallback=move || view! {
                            <div class="space-y-4">
                                <span class="text-6xl block">"🚨"</span>
                                <h2 class="text-2xl font-extrabold text-red-500">"TUDUHAN SALAH!"</h2>
                                <p class="text-sm text-slate-300">
                                    "Polisi menangkap orang yang salah! Pelaku asli menyadari pengepungan dan berhasil kabur membawa lukisan \"Monnalisa KW Super\"."
                                </p>
                                <div class="bg-slate-950 p-3 rounded-xl border border-red-900 text-xs text-slate-400 text-left">
                                    "💡 " <strong>"Tips Investigasi:"</strong>
                                    " Periksa kembali log CCTV di Misi 3 dan pastikan Anda mencocokkan id_mobil pada tabel warga dengan plat nomor di tabel kendaraan."
                                </div>
                                <button
                                    on:click=move |_| on_close.run(())
                                    class="bg-slate-800 hover:bg-slate-700 text-slate-200 font-bold py-2.5 px-6 rounded-xl text-sm transition"
                                >
                                    "🔄 Coba Investigasi Lagi"
                                </button>
                            </div>
                        }
                    >
                        <div class="space-y-4">
                            <span class="text-6xl block animate-bounce">"🏆"</span>
                            <h2 class="text-2xl font-extrabold text-blue-400">"KASUS TERSELESAIKAN!"</h2>
                            <p class="text-sm text-slate-200">
                                "Luar biasa, Detektif! Berkat keahlian query "
                                <span class="text-emerald-400 font-mono font-bold">"SQL"</span>
                                " Anda, pelaku berhasil ditangkap tepat sebelum melarikan diri di Toko Roti Sedap!"
                            </p>
                            <CulpritCard />
                            <p class="text-xs text-slate-400 italic">
                                "Anda telah menguasai dasar SELECT, WHERE, INNER JOIN, LIKE operator, GROUP BY, COUNT, ORDER BY, dan korelasi data."
                            </p>
                            <button
                                on:click=move |_| on_replay.run(())
                                class="bg-blue-500 hover:bg-blue-600 text-white font-extrabold py-2.5 px-6 rounded-xl text-sm shadow-lg transition"
                            >
                                "🎮 Main Lagi / Latihan Ulang"
                            </button>
                        </div>
                    </Show>
                </div>
            </div>
        </Show>
    }
}
