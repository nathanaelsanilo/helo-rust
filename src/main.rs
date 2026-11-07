use std::{cmp::Ordering, io};

fn main() {
    // range 1 sampai 100, termasuk 100 karena ada sama dengan (=)
    let rahasia = rand::random_range(1..=100);
    println!("tebak angka 1-100!");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("gagal baca");

        let tebakan: i32 = match input.trim().parse() {
            Ok(angka) => angka,
            Err(_) => {
                println!("ketik angka jangan huruf!");
                continue
            }
        };

        match tebakan.cmp(&rahasia) {
            Ordering::Less => println!("terlalu kecil"),
            Ordering::Greater => println!("terlalu besar"),
            Ordering::Equal => {
                println!("MATCH!!");
                break;
            }
        }

    }
}
