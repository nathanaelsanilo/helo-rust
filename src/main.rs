use std::io;

fn main() {
    println!("berapa umurmu?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("gagal baca");

    // let umur: i32 = input.trim().parse().expect("bukan angka!");
    let umur: i32 = match input.trim().parse() {
        Ok(angka) => angka,
        Err(_) => {
            println!("ketik angka. jangan huruf!");
            return;
        }
    };

    println!("tahun depan umur mu : {}", umur + 1);
}
