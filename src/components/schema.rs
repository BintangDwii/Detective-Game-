use leptos::prelude::*;

#[derive(Clone, Copy)]
struct TableDef {
    id: &'static str,
    title: &'static str,
    count: &'static str,
    cols: &'static [(&'static str, &'static str, &'static str)],
}

const TABLES: [TableDef; 4] = [
    TableDef {
        id: "warga",
        title: "📋 warga (Citizen)",
        count: "25 baris • 8 kolom",
        cols: &[
            ("id", "text-cyan-400", "INT (PK)"),
            ("nama", "text-slate-300", "VARCHAR"),
            ("jenis_kelamin", "text-slate-300", "VARCHAR"),
            ("umur", "text-slate-300", "INT"),
            ("pekerjaan", "text-slate-300", "VARCHAR"),
            ("id_mobil", "text-cyan-400", "INT (FK)"),
            ("alamat", "text-slate-300", "VARCHAR"),
            ("foto_profil", "text-emerald-400", "IMG_KEY"),
        ],
    },
    TableDef {
        id: "kendaraan",
        title: "🚗 kendaraan (Vehicles)",
        count: "20 baris • 5 kolom",
        cols: &[
            ("id_mobil", "text-cyan-400", "INT (PK)"),
            ("plat_nomor", "text-slate-300", "VARCHAR"),
            ("merk", "text-slate-300", "VARCHAR"),
            ("warna", "text-slate-300", "VARCHAR"),
            ("foto_mobil", "text-emerald-400", "IMG_KEY"),
        ],
    },
    TableDef {
        id: "tempat",
        title: "🏛️ tempat_kejadian (Locations)",
        count: "9 baris • 4 kolom",
        cols: &[
            ("id_lokasi", "text-cyan-400", "INT (PK)"),
            ("nama_tempat", "text-slate-300", "VARCHAR"),
            ("jenis_tempat", "text-slate-300", "VARCHAR"),
            ("alamat", "text-slate-300", "VARCHAR"),
        ],
    },
    TableDef {
        id: "cctv",
        title: "📹 log_cctv (Logs)",
        count: "20 baris • 5 kolom",
        cols: &[
            ("id_log", "text-cyan-400", "INT (PK)"),
            ("id_lokasi", "text-cyan-400", "INT (FK)"),
            ("waktu_kejadian", "text-slate-300", "DATETIME"),
            ("plat_nomor_terekam", "text-slate-300", "VARCHAR"),
            ("keterangan", "text-slate-300", "VARCHAR"),
        ],
    },
];

#[component]
fn SchemaTable(def: TableDef, on_insert: Callback<String>) -> impl IntoView {
    let (open, set_open) = signal(def.id == "warga");
    let col_views = def
        .cols
        .iter()
        .map(|(name, color, typ)| {
            let snippet = (*name).to_string();
            let name_class = (*color).to_string();
            let name_text = (*name).to_string();
            let typ_text = (*typ).to_string();
            view! {
                <div
                    on:click=move |_| on_insert.run(snippet.clone())
                    class="flex justify-between hover:bg-slate-800 p-1 rounded cursor-pointer transition-all"
                >
                    <span class=name_class>{name_text}</span>
                    <span class="text-slate-500">{typ_text}</span>
                </div>
            }
        })
        .collect::<Vec<_>>();
    view! {
        <div class="border border-slate-700 rounded-lg bg-slate-950 overflow-hidden">
            <button
                on:click=move |_| set_open.update(|o| *o = !*o)
                class="w-full px-3 py-2 text-left font-mono text-xs font-bold text-blue-400 bg-black/20 flex justify-between items-center"
            >
                <span>{def.title}</span>
                <span class="text-slate-500 text-[10px]">{def.count}</span>
            </button>
            <Show when=move || open.get() fallback=|| ()>
                <div class="p-2 space-y-1 text-[11px] font-mono">{col_views.clone()}</div>
            </Show>
        </div>
    }
}

#[component]
pub fn SchemaExplorer(on_insert: Callback<String>) -> impl IntoView {
    view! {
        <div class="bg-slate-800 border border-slate-700 rounded-xl p-4 shadow-md flex-1 flex flex-col">
            <h3 class="text-sm font-bold text-slate-200 mb-2 flex items-center justify-between">
                <span>"🗄️ Skema Database Polisi"</span>
                <span class="text-[10px] text-slate-400 font-normal">"Klik kolom untuk insert"</span>
            </h3>
            <p class="text-[11px] text-slate-400 mb-3">
                "Gunakan struktur tabel berikut untuk menyusun klausa "
                <span class="font-mono text-blue-400">"SELECT"</span>", "
                <span class="font-mono text-blue-400">"WHERE"</span>", dan "
                <span class="font-mono text-blue-400">"JOIN"</span>"."
            </p>
            <div class="space-y-3 overflow-y-auto max-h-[580px] pr-1">
                <For each=move || TABLES key=|t| t.id let:def>
                    <SchemaTable def=def on_insert=on_insert />
                </For>
            </div>
        </div>
    }
}
