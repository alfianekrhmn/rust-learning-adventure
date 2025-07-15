fn hello() {
    println!("Hello World")
}

fn square(x: i32) -> i32 {
    x * x    
}

fn is_even(n: u32) -> bool {
    n % 2 == 0
}

fn nilai_semester(nilai: f64) -> f64 {
    let mut dikalikan_semester = nilai * 8.0;
    dikalikan_semester / 8.0
}

pub fn run() {
    // hello();
    // let kuadrat = square(3);
    // println!("Hasil dari kuadratnya adalah {}", kuadrat)
    // let even = is_even(30);
    // println!("Apakah angkanya bisa dibagi menjadi 2? {}", even)
    
    let penilaian = nilai_semester(3.45);
    println!("Nilai IPK kamu adalah : {}", penilaian)
}