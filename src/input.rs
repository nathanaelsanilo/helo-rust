use std::io;

fn input() {
    println!("siapa namamu?");

    let mut nama = String::new(); // bikin teks kosong
    io::stdin().read_line(&mut nama).expect("gagal baca"); // baca input stdin

    println!("halo! {}", nama.trim());
}
