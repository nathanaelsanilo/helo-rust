fn main() {
    let nama = "nathan"; // sama kaya const di js
    let mut skor = 0; // mut = boleh diubah. kaya let di js
    
    skor = skor + 10;
    println!("halo {nama}, skor kamu {skor}");

    let skor = skor * 2; // shadowing: bikin variable baru nama sama
    println!("skor double: {skor}");

    println!("Hello, world!");
}
