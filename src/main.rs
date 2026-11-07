use std::io;

fn main() {
    // let umur: i32 = input.trim().parse().expect("bukan angka!");
    let umur: i32 = loop {
        println!("berapa umurmu?");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("gagal baca");

        match input.trim().parse() {
            Ok(angka) => break angka,
            Err(_) => {
                println!("ketik angka. jangan huruf!");
                continue;
            }
        }
    };

    println!("tahun depan umur mu : {}", umur + 1);
}
