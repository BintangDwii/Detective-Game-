use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::components::briefing::Briefing;
use crate::components::header::Header;
use crate::components::modals::{AccuseModal, EndgameModal};
use crate::components::pinboard::Pinboard;
use crate::components::results::Results;
use crate::components::schema::SchemaExplorer;
use crate::components::terminal::Terminal;
use crate::db;
use crate::game::{self, LEVELS};
use crate::models::{Feedback, FeedbackKind, QueryResult, Tab};

#[component]
pub fn App() -> impl IntoView {
    let level = RwSignal::new(1u8);
    let completed = RwSignal::new([false; 5]);
    let sql = RwSignal::new(String::new());
    let feedback = RwSignal::new(None::<Feedback>);
    let result = RwSignal::new(None::<QueryResult>);
    let seq = RwSignal::new(0u64);
    let tab = RwSignal::new(Tab::Table);
    let show_accuse = RwSignal::new(false);
    let verdict = RwSignal::new(None::<bool>);

    let on_run = Callback::new(move |_| {
        let q = sql.get();
        spawn_local(async move {
            match db::run_query(&q).await {
                Ok(res) => {
                    let lvl = level.get_untracked();
                    if game::validate(lvl, &res) {
                        completed.update(|c| c[(lvl - 1) as usize] = true);
                        feedback.set(Some(Feedback {
                            kind: FeedbackKind::Success,
                            message: "🎉 BUKTI DITEMUKAN! Query SQL Anda berhasil memverifikasi petunjuk kasus."
                                .to_string(),
                        }));
                    } else {
                        feedback.set(Some(Feedback {
                            kind: FeedbackKind::Info,
                            message: "✔️ Query berhasil dieksekusi. Periksa data hasil di bawah."
                                .to_string(),
                        }));
                    }
                    result.set(Some(res));
                    seq.update(|s| *s += 1);
                }
                Err(e) => {
                    feedback.set(Some(Feedback {
                        kind: FeedbackKind::Error,
                        message: format!("❌ SQL Error: {e}"),
                    }));
                }
            }
        });
    });

    let on_clear = Callback::new(move |_| sql.set(String::new()));

    let on_snippet = Callback::new(move |t: String| {
        sql.update(|s| s.push_str(&t));
    });

    let on_insert_hint = Callback::new(move |_| {
        sql.set(LEVELS[(level.get_untracked() - 1) as usize].target_hint.to_string());
    });

    let on_accuse = Callback::new(move |_| show_accuse.set(true));
    let on_cancel_accuse = Callback::new(move |_| show_accuse.set(false));

    let on_submit_accuse = Callback::new(move |(id, plat): (Option<i32>, String)| {
        show_accuse.set(false);
        verdict.set(Some(game::check_accuse(id, &plat)));
    });

    let do_reset = move || {
        level.set(1);
        completed.set([false; 5]);
        sql.set(String::new());
        feedback.set(None);
        result.set(None);
        tab.set(Tab::Table);
        show_accuse.set(false);
        verdict.set(None);
    };
    let on_reset = Callback::new(move |_| do_reset());
    let on_replay = Callback::new(move |_| do_reset());
    let on_close_endgame = Callback::new(move |_| verdict.set(None));

    view! {
        <div class="min-h-screen flex flex-col bg-slate-900 text-slate-100">
            <Header level=level on_accuse=on_accuse on_reset=on_reset />

            <main class="flex-1 w-full p-3 md:p-4 grid grid-cols-1 lg:grid-cols-12 gap-4">
                <section class="lg:col-span-3 flex flex-col gap-4">
                    <Briefing level=level completed=completed on_insert_hint=on_insert_hint />
                    <SchemaExplorer on_insert=on_snippet />
                </section>

                <section class="lg:col-span-6 flex flex-col gap-4">
                    <Terminal sql=sql on_run=on_run on_clear=on_clear on_snippet=on_snippet />
                    <Results tab=tab feedback=feedback result=result seq=seq />
                </section>

                <section class="lg:col-span-3 flex flex-col gap-4">
                    <Pinboard completed=completed on_accuse=on_accuse />
                </section>
            </main>

            <footer class="bg-slate-800 border-t border-slate-700 py-3 px-4 text-center text-xs text-slate-500">
                <p>"Detektif SQL: Proyek Mata Elang • Platform Pembelajaran Interaktif Query SQL • Rust + WebAssembly"</p>
            </footer>

            <AccuseModal
                show=show_accuse
                on_submit=on_submit_accuse
                on_cancel=on_cancel_accuse
            />
            <EndgameModal
                verdict=verdict
                on_close=on_close_endgame
                on_replay=on_replay
            />
        </div>
    }
}
