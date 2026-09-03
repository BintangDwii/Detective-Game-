use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

fn ctx2d(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .expect("2d context")
        .expect("2d context value")
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("cast to 2d")
}

fn fill(ctx: &CanvasRenderingContext2d, color: &str) {
    ctx.set_fill_style(&JsValue::from_str(color));
}

fn circle(ctx: &CanvasRenderingContext2d, x: f64, y: f64, r: f64) {
    ctx.begin_path();
    ctx.arc(x, y, r, 0.0, std::f64::consts::PI * 2.0)
        .expect("arc");
    ctx.fill();
}

/// Port dari `drawVisualAvatar()` JS: potret tersangka & kendaraan via Canvas.
pub fn draw_avatar(canvas: &HtmlCanvasElement, key: &str, name: &str) {
    let ctx = ctx2d(canvas);
    let w = canvas.width() as f64;
    let h = canvas.height() as f64;

    fill(&ctx, "#1e293b");
    ctx.fill_rect(0.0, 0.0, w, h);

    ctx.set_stroke_style(&JsValue::from_str("#334155"));
    ctx.set_line_width(4.0);
    ctx.stroke_rect(2.0, 2.0, w - 4.0, h - 4.0);

    if key.contains("mobil") {
        let body = if key.contains("merah") {
            "#ef4444"
        } else if key.contains("biru") {
            "#3b82f6"
        } else if key.contains("hijau") {
            "#22c55e"
        } else if key.contains("putih") {
            "#f1f5f9"
        } else if key.contains("silver") || key.contains("abu") {
            "#94a3b8"
        } else if key.contains("hitam") {
            "#1f2937"
        } else if key.contains("kuning") {
            "#eab308"
        } else if key.contains("cokelat") || key.contains("coklat") {
            "#92400e"
        } else {
            "#94a3b8"
        };
        fill(&ctx, body);
        ctx.fill_rect(w * 0.2, h * 0.45, w * 0.6, h * 0.3);
        fill(&ctx, "#0284c7");
        ctx.fill_rect(w * 0.3, h * 0.3, w * 0.4, h * 0.2);
        fill(&ctx, "#000000");
        circle(&ctx, w * 0.35, h * 0.75, w * 0.08);
        circle(&ctx, w * 0.65, h * 0.75, w * 0.08);
    } else {
        fill(&ctx, "#fde047");
        circle(&ctx, w / 2.0, h * 0.4, w * 0.25);

        fill(&ctx, "#0f172a");
        circle(&ctx, w / 2.0 - 10.0, h * 0.38, 3.0);
        circle(&ctx, w / 2.0 + 10.0, h * 0.38, 3.0);

        if key.contains("topi") {
            fill(&ctx, "#d97706");
            ctx.fill_rect(w / 2.0 - 25.0, h * 0.18, 50.0, 12.0);
        }
        if key.contains("jenggot") {
            fill(&ctx, "#451a03");
            ctx.begin_path();
            ctx.arc(w / 2.0, h * 0.5, 18.0, 0.0, std::f64::consts::PI)
                .expect("arc");
            ctx.fill();
        }
        if key.contains("kumis") {
            fill(&ctx, "#451a03");
            ctx.fill_rect(w / 2.0 - 14.0, h * 0.44, 28.0, 5.0);
        }
        if key.contains("helm") || key.contains("seragam") || key.contains("jaket") {
            fill(&ctx, "#334155");
            ctx.fill_rect(w / 2.0 - 25.0, h * 0.2, 50.0, 10.0);
        }
        if key.contains("kacamata") {
            ctx.set_stroke_style(&JsValue::from_str("#000000"));
            ctx.set_line_width(2.0);
            ctx.stroke_rect(w / 2.0 - 18.0, h * 0.34, 12.0, 10.0);
            ctx.stroke_rect(w / 2.0 + 6.0, h * 0.34, 12.0, 10.0);
        }

        if key.contains("wanita") {
            fill(&ctx, "#ec4899");
        } else {
            fill(&ctx, "#0284c7");
        }
        ctx.begin_path();
        ctx.arc(
            w / 2.0,
            h * 0.95,
            w * 0.38,
            std::f64::consts::PI,
            0.0,
        )
        .expect("arc");
        ctx.fill();
    }

    fill(&ctx, "rgba(15, 23, 42, 0.85)");
    ctx.fill_rect(0.0, h - 22.0, w, 22.0);
    fill(&ctx, "#f8fafc");
    ctx.set_font("bold 10px sans-serif");
    ctx.set_text_align("center");
    let label: String = name.chars().take(18).collect();
    ctx.fill_text(&label, w / 2.0, h - 7.0).expect("fill_text");
}

/// Bar chart manual (pengganti Chart.js): agregasi jumlah baris per kategori.
pub fn draw_chart(canvas: &HtmlCanvasElement, data: &[(String, usize)]) {
    let ctx = ctx2d(canvas);
    let w = canvas.width() as f64;
    let h = canvas.height() as f64;

    fill(&ctx, "#0f172a");
    ctx.fill_rect(0.0, 0.0, w, h);

    if data.is_empty() {
        return;
    }

    let max_v = data.iter().map(|(_, v)| *v).max().unwrap_or(1).max(1) as f64;
    let pad_left = 36.0;
    let pad_bottom = 44.0;
    let pad_top = 16.0;
    let plot_w = w - pad_left - 12.0;
    let plot_h = h - pad_top - pad_bottom;
    let n = data.len() as f64;
    let slot = plot_w / n;
    let bar_w = (slot * 0.55).min(64.0);

    ctx.set_font("10px sans-serif");
    ctx.set_text_align("right");

    let steps = max_v as usize;
    for i in 0..=steps {
        let v = i as f64;
        let y = pad_top + plot_h - (v / max_v) * plot_h;
        fill(&ctx, "#94a3b8");
        ctx.fill_text(&format!("{i}"), pad_left - 6.0, y + 3.0)
            .expect("fill_text");
        ctx.set_stroke_style(&JsValue::from_str("#334155"));
        ctx.set_line_width(1.0);
        ctx.begin_path();
        ctx.move_to(pad_left, y);
        ctx.line_to(w - 12.0, y);
        ctx.stroke();
    }

    ctx.set_text_align("center");
    for (i, (label, v)) in data.iter().enumerate() {
        let x = pad_left + slot * i as f64 + slot / 2.0;
        let bh = (*v as f64 / max_v) * plot_h;
        let y = pad_top + plot_h - bh;
        fill(&ctx, "#f59e0b");
        ctx.fill_rect(x - bar_w / 2.0, y, bar_w, bh);
        ctx.set_stroke_style(&JsValue::from_str("#d97706"));
        ctx.set_line_width(1.0);
        ctx.stroke_rect(x - bar_w / 2.0, y, bar_w, bh.max(1.0));

        fill(&ctx, "#f8fafc");
        ctx.fill_text(&format!("{v}"), x, y - 4.0).expect("fill_text");

        fill(&ctx, "#94a3b8");
        let short: String = label.chars().take(10).collect();
        ctx.fill_text(&short, x, pad_top + plot_h + 14.0)
            .expect("fill_text");
    }
}
