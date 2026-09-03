use std::collections::HashMap;

/// Hasil query: label kolom + baris sebagai string (NULL -> "NULL").
#[derive(Clone, Debug, Default)]
pub struct QueryResult {
    pub labels: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl QueryResult {
    pub fn col_index(&self, name: &str) -> Option<usize> {
        self.labels
            .iter()
            .position(|l| l.eq_ignore_ascii_case(name))
    }

    pub fn val(&self, row: usize, col: &str) -> Option<&str> {
        let c = self.col_index(col)?;
        self.rows.get(row)?.get(c).map(|s| s.as_str())
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FeedbackKind {
    Success,
    Error,
    Info,
}

#[derive(Clone, Debug)]
pub struct Feedback {
    pub kind: FeedbackKind,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tab {
    Table,
    Visual,
    Chart,
}

/// Satu kartu bukti visual untuk galeri.
#[derive(Clone, Debug)]
pub struct EvidenceItem {
    /// Key unik (termasuk seq query agar canvas selalu di-remount).
    pub key: String,
    pub title: String,
    pub subtitle: String,
    pub detail: String,
    pub canvas_key: String,
}

/// Turunkan daftar bukti visual dari hasil query.
///
/// Baris dianggap bukti visual jika punya foto_profil / foto_mobil / plat_nomor,
/// sama seperti filter JS asli.
pub fn derive_evidence(res: &QueryResult, seq: u64) -> Vec<EvidenceItem> {
    let mut out = Vec::new();
    for (i, _) in res.rows.iter().enumerate() {
        let foto_profil = res.val(i, "foto_profil");
        let foto_mobil = res.val(i, "foto_mobil");
        let plat = res.val(i, "plat_nomor");
        if foto_profil.is_none() && foto_mobil.is_none() && plat.is_none() {
            continue;
        }
        let title = res
            .val(i, "nama")
            .or(plat)
            .or_else(|| res.val(i, "plat_nomor_terekam"))
            .unwrap_or("Bukti")
            .to_string();
        let subtitle = res
            .val(i, "pekerjaan")
            .or_else(|| res.val(i, "merk"))
            .or_else(|| res.val(i, "keterangan"))
            .unwrap_or("Data Polisi")
            .to_string();
        let detail = res
            .val(i, "alamat")
            .or_else(|| res.val(i, "warna"))
            .or_else(|| res.val(i, "waktu_kejadian"))
            .unwrap_or("")
            .to_string();
        let canvas_key = foto_profil
            .or(foto_mobil)
            .unwrap_or("pria_topi")
            .to_string();
        out.push(EvidenceItem {
            key: format!("{seq}-{i}"),
            title,
            subtitle,
            detail,
            canvas_key,
        });
    }
    out
}

/// Agregasi untuk bar chart: kelompokkan berdasar kolom pertama yang tersedia
/// di antara pekerjaan / warna / jenis_tempat (port logika Chart.js asli).
pub fn chart_data(res: &QueryResult) -> Vec<(String, usize)> {
    let group_col = ["pekerjaan", "warna", "jenis_tempat"]
        .into_iter()
        .find(|c| res.col_index(c).is_some());
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    match group_col {
        Some(col) => {
            let ci = res.col_index(col).unwrap();
            for row in &res.rows {
                let k = row.get(ci).cloned().unwrap_or_else(|| "Lainnya".into());
                if !counts.contains_key(&k) {
                    order.push(k.clone());
                }
                *counts.entry(k).or_insert(0) += 1;
            }
            order
                .into_iter()
                .map(|k| {
                    let v = counts[&k];
                    (k, v)
                })
                .collect()
        }
        None => vec![("Baris Data".to_string(), res.row_count())],
    }
}
