use crate::models::QueryResult;

pub const CULPRIT_ID: i32 = 104;
pub const CULPRIT_PLAT: &str = "B 1299 DET";

pub struct LevelConfig {
    pub badge: &'static str,
    pub title: &'static str,
    pub story: &'static str,
    pub target_hint: &'static str,
    pub pin_title: &'static str,
    pub pin_text: &'static str,
    pub pin_locked_text: &'static str,
}

pub const LEVELS: [LevelConfig; 5] = [
    LevelConfig {
        badge: "Level 1 Active",
        title: "Petunjuk 1: Tukang Ledeng Misterius",
        story: "Saksi mata melihat pria mencurigakan keluar dari museum saat alarm berbunyi. Saksi menyebut pria itu memakai seragam <strong>Tukang Ledeng</strong>. Cari data semua warga berjenis kelamin 'Pria' yang bekerja sebagai 'Tukang Ledeng'. <em>Hati-hati: ada 1 wanita tukang ledeng dan banyak pria profesi lain sebagai pengecoh!</em>",
        target_hint: "SELECT * FROM warga WHERE jenis_kelamin = 'Pria' AND pekerjaan = 'Tukang Ledeng';",
        pin_title: "Daftar 7 Tukang Ledeng Pria Ditemukan",
        pin_text: "Salah satu dari 7 pria ini adalah pelaku! Lanjut ke Misi 2 untuk mengecek kendaraan mereka.",
        pin_locked_text: "Jalankan query Misi 1 untuk memfilter daftar tersangka pria bertopeng tukang ledeng.",
    },
    LevelConfig {
        badge: "Level 2 Active",
        title: "Petunjuk 2: Melacak Mobil Pelarian",
        story: "Kamera lalu lintas mencatat tukang ledeng mencurigakan kabur menggunakan <strong>mobil berwarna Merah</strong>. Hubungkan tabel <code class='text-amber-400'>warga</code> dengan <code class='text-amber-400'>kendaraan</code> (JOIN) untuk mencari tahu merk dan plat nomor mobil merah milik tukang ledeng! <em>Awas: ada 3 mobil merah di kota ini, hanya 1 milik tukang ledeng!</em>",
        target_hint: "SELECT w.nama, w.pekerjaan, k.merk, k.plat_nomor, k.warna FROM warga w JOIN kendaraan k ON w.id_mobil = k.id_mobil WHERE w.pekerjaan = 'Tukang Ledeng' AND k.warna = 'Merah';",
        pin_title: "Mobil Pelaku Ditentukan!",
        pin_text: "GranMax Merah (Plat: B 1299 DET) milik Eko Prasetyo. Dua mobil merah lain milik sopir & pengacara — bukan pelaku! Lanjut ke Misi 3 untuk cek log CCTV!",
        pin_locked_text: "Lakukan JOIN antara tabel warga dan kendaraan untuk menemukan plat nomor mobil merah tersangka.",
    },
    LevelConfig {
        badge: "Level 3 Active",
        title: "Petunjuk 3: Jejak CCTV Pelaku",
        story: "Plat nomor tersangka berawalan 'B 12'. Gunakan operator <code class='text-amber-400'>LIKE 'B 12%'</code> pada tabel <code class='text-amber-400'>log_cctv</code> untuk melihat lokasi mana saja yang dikunjungi mobil tersebut hari ini! <em>Ada beberapa mobil B 12xx lain — baca keterangan tiap baris untuk menemukan jejak pelaku ke pelabuhan!</em>",
        target_hint: "SELECT * FROM log_cctv WHERE plat_nomor_terekam LIKE 'B 12%';",
        pin_title: "Lokasi Persembunyi Ditemukan!",
        pin_text: "Mobil pelaku terdeteksi di bengkel, museum, toko roti, hingga pelabuhan (mau kabur!). Waktunya analisis pola, lalu tangkap!",
        pin_locked_text: "Gunakan operator LIKE pada log CCTV untuk melacak ke mana mobil berplat B 12... kabur.",
    },
    LevelConfig {
        badge: "Level 4 Active",
        title: "Petunjuk 4: Profil Pelaku — Statistik Kota",
        story: "Polda ingin memastikan pola: profesi apa yang paling mencurigakan di kota ini? Gunakan <code class='text-amber-400'>GROUP BY pekerjaan</code> + <code class='text-amber-400'>COUNT(*)</code> + <code class='text-amber-400'>ORDER BY</code> pada tabel <code class='text-amber-400'>warga</code> untuk menghitung jumlah warga per pekerjaan, urut dari terbanyak!",
        target_hint: "SELECT pekerjaan, COUNT(*) AS jumlah FROM warga GROUP BY pekerjaan ORDER BY jumlah DESC;",
        pin_title: "Pola Kriminal Terungkap!",
        pin_text: "Tukang Ledeng adalah profesi terbanyak (8 orang)! Pelaku menyamar di tengah kerumunan profesi ini. Waktunya eksekusi penangkapan!",
        pin_locked_text: "Gunakan GROUP BY + COUNT + ORDER BY untuk menemukan profesi dengan jumlah warga terbanyak.",
    },
    LevelConfig {
        badge: "Level 5 Active - FINAL",
        title: "Misi 5: Eksekusi Penangkapan",
        story: "Anda telah mengumpulkan semua bukti! Cocokkan ID Tersangka (Eko Prasetyo - ID: 104) dan plat nomor kendaraannya, lalu klik tombol <strong>'TUDUH PELAKU'</strong> di atas!",
        target_hint: "SELECT w.id, w.nama, k.plat_nomor FROM warga w JOIN kendaraan k ON w.id_mobil = k.id_mobil WHERE w.id = 104;",
        pin_title: "",
        pin_text: "",
        pin_locked_text: "",
    },
];

/// Validasi objektif tiap misi terhadap hasil query (port logika JS asli).
pub fn validate(level: u8, res: &QueryResult) -> bool {
    if res.rows.is_empty() {
        return false;
    }
    match level {
        1 => {
            // 7 pria tukang ledeng (toleransi: minimal 6 agar typo kecil tetap lolos,
            // tapi harus semuanya pria + tukang ledeng — menjebak yang lupa filter gender
            // karena ada 1 wanita tukang ledeng + banyak pria non-ledeng).
            res.rows.len() >= 6
                && (0..res.rows.len()).all(|i| {
                    res.val(i, "pekerjaan") == Some("Tukang Ledeng")
                        && res.val(i, "jenis_kelamin") == Some("Pria")
                })
        }
        2 => (0..res.rows.len()).any(|i| {
            res.val(i, "warna") == Some("Merah")
                && (res.val(i, "plat_nomor") == Some("B 1299 DET")
                    || res.val(i, "nama") == Some("Eko Prasetyo"))
        }),
        3 => (0..res.rows.len())
            .any(|i| res.val(i, "plat_nomor_terekam").is_some_and(|p| p.contains("B 1299 DET"))),
        4 => {
            // GROUP BY pekerjaan + COUNT: harus ada kolom pekerjaan & kolom agregat
            // (label GlueSQL untuk COUNT(*) bisa "COUNT(*)" atau alias "jumlah").
            let has_pekerjaan = res.col_index("pekerjaan").is_some();
            let count_col = ["jumlah", "count", "count(*)"]
                .into_iter()
                .find(|c| res.col_index(c).is_some());
            match (has_pekerjaan, count_col) {
                (true, Some(cc)) => {
                    let ci = res.col_index(cc).unwrap();
                    (0..res.rows.len()).any(|i| {
                        let is_ledeng = res.val(i, "pekerjaan") == Some("Tukang Ledeng");
                        let n: usize = res.rows.get(i).and_then(|r| r.get(ci)).and_then(|s| s.parse().ok()).unwrap_or(0);
                        is_ledeng && n >= 7
                    })
                }
                _ => false,
            }
        }
        _ => true,
    }
}

pub fn pin_unlocked_title(level: u8) -> &'static str {
    match level {
        1 => "Misi 1: Datar Tukang Ledeng",
        2 => "Misi 2: Kendaraan Pelarian",
        3 => "Misi 3: Jejak CCTV",
        4 => "Misi 4: Statistik Kota",
        _ => "",
    }
}

pub fn pin_locked_title(level: u8) -> &'static str {
    match level {
        1 => "Mencari Pria Tukang Ledeng",
        2 => "Identifikasi Mobil Merah",
        3 => "Lokasi Terakhir Pelaku",
        4 => "Pola Profesi Kriminal",
        _ => "",
    }
}

/// (id, nama, pekerjaan) untuk dropdown tuduhan — data identik dengan DB.
pub const WARGA_OPTIONS: [(i32, &str, &str); 25] = [
    (101, "Budi Santoso", "Tukang Ledeng"),
    (102, "Rudi Hermawan", "Tukang Ledeng"),
    (103, "Siti Rahma", "Guru"),
    (104, "Eko Prasetyo", "Tukang Ledeng"),
    (105, "Dewi Lestari", "Dokter"),
    (106, "Agus Setiawan", "Tukang Ledeng"),
    (107, "Bambang Wijaya", "Tukang Kayu"),
    (108, "Siska Putri", "Kasir"),
    (109, "Joko Susilo", "Tukang Ledeng"),
    (110, "Hendra Gunawan", "Tukang Ledeng"),
    (111, "Yusuf Maulana", "Tukang Ledeng"),
    (112, "Dedi Kurniawan", "Sopir"),
    (113, "Andi Nugraha", "Satpam"),
    (114, "Fajar Ramadhan", "Koki"),
    (115, "Rizky Pratama", "Ojek Online"),
    (116, "Maya Anggraini", "Perawat"),
    (117, "Ratna Sari", "Guru"),
    (118, "Fitri Handayani", "Tukang Ledeng"),
    (119, "Lina Marlina", "Pengacara"),
    (120, "Wahyu Hidayat", "Pensiunan"),
    (121, "Tono Suherman", "Pedagang"),
    (122, "Dani Saputra", "Montir"),
    (123, "Putri Ayu", "Mahasiswa"),
    (124, "Harto Wibowo", "Tukang Kayu"),
    (125, "Novi Astuti", "Barista"),
];

pub fn check_accuse(suspect_id: Option<i32>, plat: &str) -> bool {
    suspect_id == Some(CULPRIT_ID)
        && plat.to_uppercase().contains(CULPRIT_PLAT)
}
