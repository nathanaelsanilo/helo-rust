use std::{cmp::Ordering, io};

fn main() {
    // range 1 sampai 100, termasuk 100 karena ada sama dengan (=)
    let rahasia = rand::random_range(1..=100);
    let mut percobaan = 0;
    let maks = 7;

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

        percobaan += 1;

        match tebakan.cmp(&rahasia) {
            Ordering::Less => println!("terlalu kecil"),
            Ordering::Greater => println!("terlalu besar"),
            Ordering::Equal => {
                println!("MATCH!! kamu menang dalam {percobaan} tebakan!");
                break;
            }
        }

        if percobaan >= maks {
            println!("Game Over! Angkanya adalah {rahasia}");
            break;
        }

    }
}
