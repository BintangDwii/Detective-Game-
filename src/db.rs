use gluesql_core::prelude::{Glue, Payload, Value};
use gluesql_memory_storage::MemoryStorage;

use crate::models::QueryResult;

const SCHEMA: &[&str] = &[
    "CREATE TABLE warga (id INT, nama TEXT, jenis_kelamin TEXT, umur INT, pekerjaan TEXT, id_mobil INT, alamat TEXT, foto_profil TEXT)",
    "CREATE TABLE kendaraan (id_mobil INT, plat_nomor TEXT, merk TEXT, warna TEXT, foto_mobil TEXT)",
    "CREATE TABLE tempat_kejadian (id_lokasi INT, nama_tempat TEXT, jenis_tempat TEXT, alamat TEXT)",
    "CREATE TABLE log_cctv (id_log INT, id_lokasi INT, waktu_kejadian TEXT, plat_nomor_terekam TEXT, keterangan TEXT)",
];

const SEED: &[&str] = &[
    "INSERT INTO warga VALUES (101, 'Budi Santoso', 'Pria', 38, 'Tukang Ledeng', 201, 'Jl. Merdeka No. 12', 'pria_topi_kumis')",
    "INSERT INTO warga VALUES (102, 'Rudi Hermawan', 'Pria', 42, 'Tukang Ledeng', 202, 'Jl. Mawar No. 45', 'pria_kacamata')",
    "INSERT INTO warga VALUES (103, 'Siti Rahma', 'Wanita', 29, 'Guru', 203, 'Jl. Anggrek No. 8', 'wanita_rambut_panjang')",
    "INSERT INTO warga VALUES (104, 'Eko Prasetyo', 'Pria', 35, 'Tukang Ledeng', 204, 'Jl. Diponegoro No. 88', 'pria_jenggot')",
    "INSERT INTO warga VALUES (105, 'Dewi Lestari', 'Wanita', 31, 'Dokter', 205, 'Jl. Sudirman No. 101', 'wanita_kacamata')",
    "INSERT INTO warga VALUES (106, 'Agus Setiawan', 'Pria', 45, 'Tukang Ledeng', 206, 'Jl. Pemuda No. 19', 'pria_rambut_pendek')",
    "INSERT INTO warga VALUES (107, 'Bambang Wijaya', 'Pria', 50, 'Tukang Kayu', 207, 'Jl. Melati No. 3', 'pria_topi')",
    "INSERT INTO warga VALUES (108, 'Siska Putri', 'Wanita', 26, 'Kasir', NULL, 'Jl. Dahlia No. 7', 'wanita_pendek')",
    // --- Suspect tambahan: decoy Tukang Ledeng pria (Misi 1 jadi 7 orang) ---
    "INSERT INTO warga VALUES (109, 'Joko Susilo', 'Pria', 39, 'Tukang Ledeng', 208, 'Jl. Kenanga No. 22', 'pria_kumis')",
    "INSERT INTO warga VALUES (110, 'Hendra Gunawan', 'Pria', 33, 'Tukang Ledeng', 209, 'Jl. Cendana No. 5', 'pria_rambut_pendek')",
    "INSERT INTO warga VALUES (111, 'Yusuf Maulana', 'Pria', 47, 'Tukang Ledeng', 210, 'Gg. Mangga No. 14', 'pria_topi')",
    // --- Pria non-ledeng (profesi realistis) ---
    "INSERT INTO warga VALUES (112, 'Dedi Kurniawan', 'Pria', 29, 'Sopir', 211, 'Jl. Supratman No. 30', 'pria_helm')",
    "INSERT INTO warga VALUES (113, 'Andi Nugraha', 'Pria', 36, 'Satpam', 212, 'Jl. Garuda No. 11', 'pria_seragam')",
    "INSERT INTO warga VALUES (114, 'Fajar Ramadhan', 'Pria', 27, 'Koki', 213, 'Jl. Seroja No. 9', 'pria_koki')",
    "INSERT INTO warga VALUES (115, 'Rizky Pratama', 'Pria', 24, 'Ojek Online', NULL, 'Jl. Flamboyan No. 2', 'pria_jaket')",
    // --- Warga wanita (termasuk 1 jebakan gender: wanita tukang ledeng!) ---
    "INSERT INTO warga VALUES (116, 'Maya Anggraini', 'Wanita', 34, 'Perawat', 214, 'Jl. Teratai No. 17', 'wanita_perawat')",
    "INSERT INTO warga VALUES (117, 'Ratna Sari', 'Wanita', 28, 'Guru', 215, 'Jl. Kamboja No. 33', 'wanita_rambut_panjang')",
    "INSERT INTO warga VALUES (118, 'Fitri Handayani', 'Wanita', 37, 'Tukang Ledeng', 216, 'Jl. Nusa Indah No. 6', 'wanita_helm')",
    "INSERT INTO warga VALUES (119, 'Lina Marlina', 'Wanita', 41, 'Pengacara', 217, 'Jl. Veteran No. 58', 'wanita_kacamata')",
    // --- Campuran realistis (ada yang tidak punya mobil) ---
    "INSERT INTO warga VALUES (120, 'Wahyu Hidayat', 'Pria', 52, 'Pensiunan', NULL, 'Jl. Pahlawan No. 40', 'pria_bald')",
    "INSERT INTO warga VALUES (121, 'Tono Suherman', 'Pria', 44, 'Pedagang', 218, 'Jl. Pasar Baru No. 15', 'pria_kumis')",
    "INSERT INTO warga VALUES (122, 'Dani Saputra', 'Pria', 31, 'Montir', 219, 'Jl. Otomotif No. 8', 'pria_jenggot')",
    "INSERT INTO warga VALUES (123, 'Putri Ayu', 'Wanita', 23, 'Mahasiswa', NULL, 'Jl. Pendidikan No. 21', 'wanita_pendek')",
    "INSERT INTO warga VALUES (124, 'Harto Wibowo', 'Pria', 55, 'Tukang Kayu', 220, 'Jl. Jati No. 77', 'pria_topi')",
    "INSERT INTO warga VALUES (125, 'Novi Astuti', 'Wanita', 30, 'Barista', NULL, 'Jl. Kopi No. 13', 'wanita_barista')",
    "INSERT INTO kendaraan VALUES (201, 'B 4412 XYZ', 'Toyota Avanza', 'Hitam', 'mobil_hitam')",
    "INSERT INTO kendaraan VALUES (202, 'B 8831 KLM', 'Honda Jazz', 'Biru', 'mobil_biru')",
    "INSERT INTO kendaraan VALUES (203, 'B 9012 EFG', 'Daihatsu Ayla', 'Putih', 'mobil_putih')",
    "INSERT INTO kendaraan VALUES (204, 'B 1299 DET', 'Daihatsu GranMax', 'Merah', 'mobil_merah')",
    "INSERT INTO kendaraan VALUES (205, 'B 5500 LMN', 'Mitsubishi Pajero', 'Hitam', 'mobil_hitam')",
    "INSERT INTO kendaraan VALUES (206, 'B 3311 OPQ', 'Suzuki Carry', 'Hijau', 'mobil_hijau')",
    "INSERT INTO kendaraan VALUES (207, 'B 7722 RST', 'Toyota Innova', 'Silver', 'mobil_silver')",
    // --- Kendaraan tambahan (termasuk 2 red herring merah + 2 decoy B 12xx) ---
    "INSERT INTO kendaraan VALUES (208, 'B 7721 JKS', 'Toyota Kijang', 'Putih', 'mobil_putih')",
    "INSERT INTO kendaraan VALUES (209, 'B 5543 HDU', 'Daihatsu Xenia', 'Silver', 'mobil_silver')",
    "INSERT INTO kendaraan VALUES (210, 'B 8810 PQR', 'Suzuki APV', 'Hitam', 'mobil_hitam')",
    "INSERT INTO kendaraan VALUES (211, 'B 1190 ZXC', 'Honda Brio', 'Merah', 'mobil_merah')",
    "INSERT INTO kendaraan VALUES (212, 'B 1205 AAA', 'Daihatsu Sigra', 'Putih', 'mobil_putih')",
    "INSERT INTO kendaraan VALUES (213, 'B 3345 BCD', 'Toyota Calya', 'Abu-abu', 'mobil_silver')",
    "INSERT INTO kendaraan VALUES (214, 'B 7788 QRS', 'Honda Mobilio', 'Hitam', 'mobil_hitam')",
    "INSERT INTO kendaraan VALUES (215, 'B 1233 BBB', 'Toyota Rush', 'Kuning', 'mobil_kuning')",
    "INSERT INTO kendaraan VALUES (216, 'B 9015 HIJ', 'Nissan Livina', 'Putih', 'mobil_putih')",
    "INSERT INTO kendaraan VALUES (217, 'B 4820 QWE', 'Toyota Yaris', 'Merah', 'mobil_merah')",
    "INSERT INTO kendaraan VALUES (218, 'B 6678 EFG', 'Mitsubishi L300', 'Cokelat', 'mobil_cokelat')",
    "INSERT INTO kendaraan VALUES (219, 'B 4456 NOP', 'Suzuki Ertiga', 'Silver', 'mobil_silver')",
    "INSERT INTO kendaraan VALUES (220, 'B 2210 KLM', 'Isuzu Panther', 'Hijau', 'mobil_hijau')",
    "INSERT INTO tempat_kejadian VALUES (301, 'Museum Kota', 'Museum', 'Jl. Museum No. 1')",
    "INSERT INTO tempat_kejadian VALUES (302, 'Kafe Kopi Kenangan', 'Kafe', 'Jl. Melati No. 15')",
    "INSERT INTO tempat_kejadian VALUES (303, 'Toko Roti Sedap', 'Toko Roti', 'Jl. Diponegoro No. 85')",
    "INSERT INTO tempat_kejadian VALUES (304, 'Stasiun Kereta Utama', 'Stasiun', 'Jl. Stasiun No. 99')",
    "INSERT INTO tempat_kejadian VALUES (305, 'Bank Sentral Nasional', 'Bank', 'Jl. Merdeka No. 50')",
    "INSERT INTO tempat_kejadian VALUES (306, 'Pasar Induk Kramat', 'Pasar', 'Jl. Kramat No. 77')",
    "INSERT INTO tempat_kejadian VALUES (307, 'Bengkel Jaya Abadi', 'Bengkel', 'Jl. Otomotif No. 21')",
    "INSERT INTO tempat_kejadian VALUES (308, 'Hotel Grand Mawar', 'Hotel', 'Jl. Mawar No. 99')",
    "INSERT INTO tempat_kejadian VALUES (309, 'Pelabuhan Tanjung Baru', 'Pelabuhan', 'Jl. Pelabuhan No. 5')",
    "INSERT INTO log_cctv VALUES (401, 301, '2026-09-03 08:30:00', 'B 4412 XYZ', 'Melintas biasa')",
    "INSERT INTO log_cctv VALUES (402, 301, '2026-09-03 09:15:00', 'B 1299 DET', 'Parkir di belakang museum selama 20 menit')",
    "INSERT INTO log_cctv VALUES (403, 302, '2026-09-03 09:40:00', 'B 8831 KLM', 'Membeli kopi drive-thru')",
    "INSERT INTO log_cctv VALUES (404, 303, '2026-09-03 10:10:00', 'B 1299 DET', 'Mobil merah mematikan lampu di area parkir')",
    "INSERT INTO log_cctv VALUES (405, 304, '2026-09-03 10:45:00', 'B 9012 EFG', 'Menurunkan penumpang')",
    // --- Log tambahan: jejak pelaku + decoy B 12xx + lalu-lintas realistis ---
    "INSERT INTO log_cctv VALUES (406, 307, '2026-09-03 07:20:00', 'B 1299 DET', 'Membeli perkakas dan selang air di bengkel')",
    "INSERT INTO log_cctv VALUES (407, 305, '2026-09-03 07:45:00', 'B 1205 AAA', 'Tarik tunai di ATM drive-thru bank')",
    "INSERT INTO log_cctv VALUES (408, 301, '2026-09-03 08:05:00', 'B 7721 JKS', 'Parkir singkat, sopir membeli koran')",
    "INSERT INTO log_cctv VALUES (409, 304, '2026-09-03 08:50:00', 'B 1233 BBB', 'Mengantar penumpang ke stasiun')",
    "INSERT INTO log_cctv VALUES (410, 302, '2026-09-03 09:05:00', 'B 1190 ZXC', 'Membeli kopi, mobil merah lain')",
    "INSERT INTO log_cctv VALUES (411, 307, '2026-09-03 09:30:00', 'B 4456 NOP', 'Servis rutin kendaraan di bengkel')",
    "INSERT INTO log_cctv VALUES (412, 306, '2026-09-03 09:55:00', 'B 6678 EFG', 'Bongkar muat barang dagangan di pasar')",
    "INSERT INTO log_cctv VALUES (413, 303, '2026-09-03 10:25:00', 'B 1299 DET', 'Mobil merah terlihat melambat di depan toko roti')",
    "INSERT INTO log_cctv VALUES (414, 308, '2026-09-03 10:35:00', 'B 4820 QWE', 'Check-in tamu hotel, mobil merah')",
    "INSERT INTO log_cctv VALUES (415, 309, '2026-09-03 11:00:00', 'B 1299 DET', 'Terdeteksi menuju area pelabuhan, dicurigai mau kabur')",
    "INSERT INTO log_cctv VALUES (416, 301, '2026-09-03 11:10:00', 'B 5543 HDU', 'Rombongan turis parkir di area museum')",
    "INSERT INTO log_cctv VALUES (417, 305, '2026-09-03 11:25:00', 'B 1205 AAA', 'Terlihat lagi di area bank, menyetor tunai')",
    "INSERT INTO log_cctv VALUES (418, 304, '2026-09-03 11:40:00', 'B 9015 HIJ', 'Menjemput penumpang di stasiun')",
    "INSERT INTO log_cctv VALUES (419, 306, '2026-09-03 11:55:00', 'B 8810 PQR', 'Belanja suku cadang di pasar')",
    "INSERT INTO log_cctv VALUES (420, 309, '2026-09-03 12:10:00', 'B 4456 NOP', 'Antar jemput logistik pelabuhan')",
];

fn value_to_string(v: Value) -> String {
    match v {
        Value::Str(s) => s,
        Value::Null => "NULL".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::I64(n) => n.to_string(),
        Value::F64(f) => {
            if f.fract() == 0.0 {
                format!("{}", f as i64)
            } else {
                f.to_string()
            }
        }
        other => format!("{other:?}"),
    }
}

/// Jalankan satu query SELECT pengguna terhadap database polisi.
///
/// Database dibangun ulang (fresh + seed) setiap eksekusi — murah (~70 baris)
/// dan membuat tiap query stateless, sama seperti interpreter JS asli yang
/// selalu menyalin ulang objek DB.
pub async fn run_query(sql: &str) -> Result<QueryResult, String> {
    let trimmed = sql.trim();
    let trimmed = trimmed.strip_suffix(';').unwrap_or(trimmed).trim();

    if trimmed.is_empty() {
        return Err("Mohon masukkan perintah SQL terlebih dahulu.".to_string());
    }
    if trimmed.len() < 6 || !trimmed[..6].eq_ignore_ascii_case("select") {
        return Err("Saat ini simulator hanya mendukung perintah SELECT.".to_string());
    }

    let mut glue: Glue<MemoryStorage> = Glue::new(MemoryStorage::default());
    for stmt in SCHEMA.iter().chain(SEED.iter()) {
        glue.execute(*stmt)
            .map_err(|e| format!("DB init error: {e}"))?;
    }

    let payloads = glue.execute(trimmed).map_err(|e| format!("{e}"))?;

    for payload in payloads {
        if let Payload::Select { labels, rows } = payload {
            let rows = rows
                .into_iter()
                .map(|r| r.into_iter().map(value_to_string).collect())
                .collect();
            return Ok(QueryResult { labels, rows });
        }
    }

    Err("Query tidak mengembalikan baris data (hanya SELECT yang didukung).".to_string())
}
