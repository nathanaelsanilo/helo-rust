# Rangkuman Belajar: Game Tebak Angka (7 Nov 2026)

****

Sesi pertama belajar Rust, sekitar 1 jam 25 menit. Dimulai dari nol (belum install) sampai punya game tebak angka yang jalan.

# Yang dikerjakan

- Install Rust lewat `rustup`, cek dengan `rustc --version` dan `cargo --version`
- `cargo new halo-rust`, lalu `cargo run` untuk Hello World pertama
- Belajar bertahap: baca input, ubah teks jadi angka, tangani error, loop, bandingkan angka
- Jadi game tebak angka dengan angka acak, hitungan percobaan, dan batas kesempatan

# Konsep baru hari ini

**Input dari keyboard**

- `String::new()` bikin teks kosong, harus `mut` karena akan diisi
- `io::stdin().read_line(&mut input)` membaca satu baris dan menaruhnya ke `input`
- Karakter Enter (`\n`) ikut tersimpan, jadi pakai `.trim()` untuk membuangnya
- `&mut` = meminjamkan variabel supaya boleh diubah oleh fungsi lain (detailnya nanti di materi ownership dan borrowing)

**Parse dan Result**

- `parse()` mengubah teks jadi angka, tipe tujuan ditulis di variabel: `let umur: i32 = ...`
- Hasilnya `Ok(nilai)` atau `Err(...)`, bukan exception seperti Java
- `expect("pesan")`: kalau gagal, program panic dan berhenti. Cocok untuk sementara saja
- `match` dipakai untuk menangani `Ok` dan `Err` tanpa crash

**Loop yang mengembalikan nilai**

```rust
let umur: i32 = loop {
    // ...
    match input.trim().parse() {
        Ok(angka) => break angka,   // keluar dan bawa nilai
        Err(_) => continue,          // ulangi
    }
};
```

**Membandingkan angka**

- `a.cmp(&b)` menghasilkan `Ordering::Less`, `Ordering::Greater`, atau `Ordering::Equal`
- Perlu `use std::cmp::Ordering;`

**Dependency**

- `cargo add rand` menambah library ke `Cargo.toml` (setara `package.json` / `pom.xml`)
- Angka acak: `rand::random_range(1..=100)`. `1..=100` termasuk 100, `1..100` tidak
- Versi yang terpasang: rand 0.10.x

**Lainnya**

- `percobaan += 1` untuk menambah counter (variabel harus `mut`)
- `if` tanpa kurung, kurung kurawal wajib
- Shadowing dipakai lagi: `tebakan` berubah dari `String` menjadi `i32` dengan nama yang sama