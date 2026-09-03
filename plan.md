# Detektif SQL: Proyek Mata Elang — Rust WASM Implementation Plan

## 1. Arsitektur
- **Frontend**: Leptos 0.8 (CSR) + Trunk, target `wasm32-unknown-unknown`
- **SQL Engine**: GlueSQL + MemoryStorage (pure Rust WASM)
- **Visual**: Canvas API via `web-sys` (avatar/mobil + bar chart), Chart.js dihapus
- **Styling**: Tailwind CDN + Google Fonts (sama seperti HTML asli)

## 2. Substitusi penting
- `rusqlite` → **GlueSQL** (tidak ada C dependency, murni WASM-compatible)
- `sqlparser-rs` + execution layer nyata, bukan string-matching JS
- Hint Misi 4 diperbaiki (tambah `JOIN ... ON` karena SQL valid wajib)
- `LIKE` GlueSQL case-sensitive, hint sudah pakai huruf besar benar

## 3. Struktur proyek
- `src/db.rs` — CREATE TABLE ×4 + INSERT seed + query()
- `src/models.rs` — Row, EvidenceItem
- `src/game.rs` — LEVEL_CONFIGS 1–4, validate(), accuse, reset
- `src/canvas.rs` — drawVisualAvatar(), draw_bar_chart()
- `src/components/` — header, briefing, schema, terminal, results, pinboard, modals
- `src/app.rs` — layout + wiring

## 4. Tahapan
1. Setup (Cargo.toml, index.html, Trunk.toml, main.rs)
2. Engine DB (db.rs, models.rs)
3. Game logic (game.rs)
4. Canvas (canvas.rs)
5. UI components (components/*)
6. App wiring (app.rs)
7. Build verification (trunk build --release)
