use core::slice;
use std::{f32::consts::E, thread::ThreadId};

#[test]
fn hello_test() {
    println!("Hello test")
} 

#[test]
fn test_variable(){
    let fullname = "Alfian Eka Rahman";
    println!("Nama saya {}", fullname);
}

#[test]
fn test_mutable(){
    let mut name = "Mike Wazowsky";
    println!("Nama saya {}", name);

    name = "George Houshin";
    println!("Nama saya {}", name);
}


#[test]
fn static_typing(){
    let name = "Mike Wazowsky";
    println!("Nama saya {}", name);

    // name = "George Houshin";
    println!("Nama saya {}", name);
}


#[test]
fn shadowing(){
    let name = "Mike Wazowsky";
    println!("Nama saya {}", name);

    let name = 10;
    println!("Nama saya {}", name); 
}



/*
    1
    2
    3
*/
#[test]
fn comment(){
    //ini komentar
    let name = "Mike Wazowsky";
    println!("Nama saya {}", name); //ini komentar

    let name = 10;
    println!("Nama saya {}", name); 
}

#[test]
fn explicit(){
    let age: i32 = 32;
    println!("{}", age)
}

#[test]
fn number(){
    let a = 10;
    println!("{}", a);

    let b: f64 = 1.2;
    print!("{}", b);
}

#[test]
fn number_conversion(){
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let  d: i64 = 100000000;
    let e: i8 = d as i8;
    println!("{}", e)
}

#[test]
fn numerik_operator(){
    let a = 12;
    let b = 98;
    let c = a + b;
    println!("{}", c);
    let d = a - b;
    println!("{}", d);
    let e = a * b;
    println!("{}", e);
}

#[test]
fn augmented_assignment(){
    let mut a = 23;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean(){
    let a = true;
    let b = false;
    println!("{} {}", a, b);
}

#[test]
fn comparison(){
    let a = 20;
    let b = 33;
    let result = a > b;
    println!("{}", result);
}

#[test]
fn boolean_operator(){
    let nilai_presensi = 80;
    let nilai_tugas = 70;

    let cek_presensi = nilai_presensi >= 85;
    let cek_tugas = nilai_tugas >= 85;

    let cek_kelulusan = cek_presensi && cek_tugas;
    println!("{}", cek_kelulusan);
}

#[test]
fn char_type(){
    let char1 = 'a';
    let char2 = 'b';
    println!("{} {}", char1, char2);
}

#[test]
fn tuple(){
    let mut data = (10, 10.2, true);
    println!("{:?}", data);
    
    data.0 = 88;
    data.1 = 98.98;
    data.2 = false;

    println!("{:?}", data);
}


fn unit(){
    println!("hello world");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);
    
    let test = ();
    println!("{:?}", test);
}

#[test]
fn array(){
    let mut array = [1, 2, 3, 4];
    println!("{:?}", array);
    
    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 11;
    array[1] = 12;
    array[2] = 13;
    array[3] = 14;
    println!("{:?}", array);

    let length = array.len();
    println!("{}", length)
}

#[test]
fn two_demonsional_array(){
    let matrix = [[1, 2], [3, 4]];
    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant(){
    const MINIMUM: i32 = 10;
    println!("{} {}", MINIMUM, MAXIMUM);
}

#[test]
fn variable_scope(){
    let name = "Alfian";
    println!("Nama saya adalah {}", name);
    {
        let _umur = 21;
        println!( "{} {}", name," adalah seorang mahasiswa");
    }
    // println!("Umur saya adalah {}", umur) Eror
}

#[test]
fn stack_heap(){
    function_a();
    function_b();
}

fn function_b(){
    let a = 10;
    let b = String::from("Alfian");

    println!("{} {}", a, b);
}

fn function_a(){
    let a = 10;
    let b = String::from("Eka");
    println!("{} {}", a, b);
}


#[test]
fn string(){
    let name = " Alfian Eka Rahman ";
    let trim = name.trim();

    println!("{} {} {}", trim, trim.len(), name.len());

    let mut username = "Mike";
    println!("{}", username);
    
    username = "Jhon";
    println!("{}", username);
}

#[test]
fn string_type(){
    let mut name = String::from("Alfian Eka");
    println!("{}", name);

    name.push_str(" Rahman");
    println!("{}", name);
}

#[test]
fn ownership_rules(){
    let a = 10;
    
    {
        let b = 23;
        println!("{}", b);
    }
    println!("{}", a);
}

#[test]
fn data_copy(){
    let a = 10;
    let b = a;

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement(){
    let name1 = String::from("Alfian");

    let _name2 = name1;

    println!("{}", _name2);
}

#[test]
fn clone(){
    let fullname = String::from("Alfian Eka Rahman");
    let myname = fullname.clone();

    println!("{} {}",fullname, myname );
}

#[test]
fn if_expression(){
    let value = 20;
    let result = if value > 9{
        "GooD!"
    }else if value >= 7{
        "Not Bad!"
    }else if value >= 4 {
        "Bad!"
    }else {
        "Very Bad!"
    };
    
    println!("{}", result);
}

#[test]
fn loop_expression(){
    let mut counter = 0;
    loop {
        counter += 1;
        
        if counter > 10 {
            break;
        }else if counter % 2 == 0 {
            continue;
        }
        println!("{}", counter)
    }
}

#[test]
fn loop_return_value(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}

#[test]
fn loop_label(){
    let mut count = 0;
    'counting_up: loop {
        let mut  remaining = 1;
        loop {
        if remaining <= 10 {
            break;
        }
        if count == 10 {
            break 'counting_up;
            
        }
        remaining += 1;
        }
        count += 1;
        println!("{} * {} = {}", count, remaining, count * remaining);
    }
}

#[test]
fn array_iteration() {
    let array = ["a", "b", "c", "d", "e"];
    let mut index = 0;
    while index < array.len() {
        println!("Value: {}", array[index]);
        index += 1;
    }

    let arr = ["p", "o", "i", "u"];
    for value in arr {
        println!("The value is {}", value);
    }
}

#[test]
fn range() {
    let range = 0..5;
    println!("Start : {}", range.start);
    println!("End : {}", range.end);

    let array = ["l", "k", "j", "h", "g"];
    for i in range {
        println!("{}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    let range = 0..=4;
    println!("Start : {}", range.start());
    println!("End : {}", range.end());

    let array = ["l", "k", "j", "h", "g"];
    for i in range {
        println!("{}", array[i]);
    }
}

fn say_hello() {
    println!("Hello world")
}

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
    say_hello();
}

fn say_goodbye(firstname: &str, lastname: &str){
    println!("Goodbye {} {}", firstname, lastname);
}

#[test]
fn test_say_goodbye() {
    say_goodbye("Alfian", "Eka Rahman");
}


fn factorial_loop(n: i32) -> i32{
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i
    }
    result
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(10);
    println!("{}", result);
    
    let result = factorial_loop(7);
    println!("{}", result);

    let result = factorial_loop(-12);
    println!("{}", result);
}

fn print_text(value: String, time: u32) {
    if time == 0 {
        return;
    }else {
        println!("{}", value);
    }
    print_text(value, time - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("Alfian"), 5);
}

fn recursive_factorial(n: i32) -> i32{
    if n <= 1 {
        return 1;
    }
    
    n * recursive_factorial(n - 1)
}

#[test]
fn test_recursive_factorial() {
    let recursive = recursive_factorial(3);
    println!("{}", recursive);
}

fn print_number(n: i32){
    println!("Number {}", n);
}

fn hi(name:String) {
    println!("Hi {}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Mike");
    hi(name);
    // print!("Hi {}", name);
}

#[test]
fn test_main() {
    let nama = "Alfian";
    let mut umur = 21;
    println!("Nama saya adalah {}", nama);
    println!("Umur saya adalah {}", umur);

    umur = 20;
    println!("Umur saya sekarang baru {}", umur)
} 

fn variable() {
    let nama: &'static str = "Alfian";
    let mut umur: i32 = 22;
    let tinggi: f64 = 165.5;
    let status_mahasiswa: bool = true;
    let karakter_kesukaan: char = 'A';

    println!("Nama saya adalah {}", nama);
    println!("Umur saya adalah {} tahun", umur);
    println!("Tinggi saya adalah {}cm", tinggi);
    println!("Apakah saya adalah mahasiswa? {}", status_mahasiswa);
    println!("Karakter yang saya sukai adalah '{}'", karakter_kesukaan);
}

fn tipe_data() {
    let umur: u8 = 25;
    let tinggi = 170.5;
    let aktif = true;
    let huruf = 'R';

    println!("Umur : {}, Tinggi {}, Aktif : {}, Huruf : {}",umur, tinggi, aktif, huruf )
}

fn konversi_type() {
    let x = 42;
    let y = x as f64;

    println!("Nilai X: {}, setelah casting ke float: {}", x, y);
}

fn casting() {
    let panjang = 20.5;
    let lebar = 35.5;
    let luas = panjang * lebar;
    println!("Panjangnya {}", panjang);
    println!("Lebarnya {}", lebar);
    println!("Luas {}", luas);
    println!("Luas yang sudah dibulatkan {}", luas as i32);
    let info_persegi_panjang = (panjang, lebar, luas);
    println!("{:#?}", info_persegi_panjang);
}

fn data_mahasiswa() {
    let nama = "Alfian";
    let umur: u8 = 20;
    let ipk: f32 = 3.75;
    let status_mahasiswa = true;
    let info_mahasiswa = (nama, umur, ipk, status_mahasiswa);

    println!("{:#?}", info_mahasiswa);
}

fn cek_nilai_if(nilai: u8) {
    if nilai >= 90 {
        println!("Dengan nilai {}, kamu mendapat predikat: A", nilai)
    }else if nilai >= 80 {
        println!("Dengan nilai {}, kamu mendapat predikat: B", nilai)
    }else if nilai >= 70 {
        println!("Dengan nilai {}, kamu mendapat predikat: C", nilai)
    }else {
        println!("Dengan nilai {}, kamu mendapat predikat: D", nilai)
    }
}

fn cek_nilai_switch(nilai: u8) {
    match nilai {
        90..=100 => println!("Dengan nilai {}, kamu mendapat predikat: A", nilai),
        80..=89 => println!("Dengan nilai {}, kamu mendapat predikat: B", nilai),
        70..=79 => println!("Dengan nilai {}, kamu mendapat predikat: C", nilai),
        _ => println!("Dengan nilai {}, kamu mendapat predikat: D", nilai),
    }
}

fn kategori_usia(usia: u8) {
    match usia {
        0..=12 => println!("Usia {} termasuk dalam kategori : Anak - Anak", usia),
        13..=17 => println!("Usia {} termasuk dalam kategori : Remaja", usia),
        18..=59 => println!("Usia {} termasuk dalam kategori : Dewasa", usia),
        _ => println!("Usia {} termasuk dalam kategori : Lansia", usia),
    }
}


fn cetak_ganjil(angka: i32) {
    for nomor in 1..=angka {
        if nomor % 2 != 0 {
            println!("{} adalah angka ganjil", nomor)
        }
    }
}

fn mundur_angka(angka: i32) {
    let mut nomor = angka;
    while nomor > 0{
        println!("{}", nomor);
        nomor -= 1
    }
}

fn looping_nama(mut jumlah: i32) {
    loop {
        println!("Nama saya adalah Rustacean!");
        jumlah -= 1;
        if jumlah == 0 {
            break;
        }
    }
} 

fn tampilkan_nilai() {
    let array_nilai_ujian = [80, 88, 65, 76, 98];
    for nilai in array_nilai_ujian {
        println!("Nilai Akhir Mahasiswa Adalah {}", nilai)
    }
}

fn keranjang_belanja() {
    let mut all_product = vec!["Cutterpilar", "Sosis", "Kopi Susu", "Bronis Lumer", "Pizza"];
    all_product.push("Kopi Vanila");

    println!("Daftar Product");
    for product in all_product {
        println!("- {:?}", product)
    }
}

// fn kali_angka(a:i32, b:i32) -> i32 {
//     a * b
// }

// fn cek_kelulusan(nilai: u8) -> bool{
//     nilai >= 75
// }

// fn profil_mahasiswa(nama: &str, umur: u8, ipk: f32) -> (String, u8, f32) {
//     (nama.to_string(), umur, ipk)
// }

// struct Mahasiswa12 {
//     nama: String,
//     umur: u8,
//     ipk: f32,
// }

// struct Produk {
//     nama: String,
//     harga: u32,
//     stok: u16,
// }

// fn tampilkan_produk(produk: &Produk){
//     println!("Nama : {}", produk.nama);
//     println!("Harga : {}", produk.harga);
//     println!("Stok : {}", produk.stok);
// }

// struct Mahasiswa {
//     nama: String,
//     ipk: f32,
//     aktif: bool,
// }

// impl Mahasiswa {
//     fn perkenalan_mahasiswa(&self) {
//         println!("Halo perkenalkan nama saya {}. ipk saya adalah {}.", self.nama, self.ipk);
//     }

//     fn status(&self) {
//         if self.aktif {
//             println!("Status Mahasiswa : Aktif");
//         }else {
//             println!("Status Mahasiswa : Tidak Aktif");
//         }
//     }
// }

// struct Produk {
//     nama: String,
//     harga: u32,
//     stok: u32,
// }

// impl Produk {
//     fn tampilkan(&self) {
//         println!("Produk {} akan dijual dengan harga {} dan jumlah stoknya sebanyak {} buah", self.nama, self.harga, self.stok);
//     }

//     fn tambah_stok(&mut self, jumlah: u32) {
//         self.stok += jumlah;
//         println!("Jumlah stok {} menjadi {} buah", self.nama, self.stok);
//     }

//     fn diskon(&mut self, persen: u32) {
//         let potongan = self.harga * persen / 100;
//         self.harga -= potongan;
//         println!("Produk {} kini menjadi diskon {}% dengan harga {}", self.nama, persen, self.harga);
//     }
// }

// fn cetak_bio(bio: &String) {
//     println!("{}", bio);
// }

// fn ubah_status_mahasiswa(status: &mut bool) {
//     *status = !*status
// }


// fn ambil_nama_depan(nama: String){
//     let depan = &nama[0..6];
//     println!("Nama depan: {}", depan); // Output: Alfian
// }

// enum Status {
//     Aktif,
//     TidakAktif,
// }

// fn cek_status(status: Status) {
//     match status {
//         Status::Aktif => println!("Mahasiswa sedang aktif"),
//         Status::TidakAktif => println!("Mahasiswa tidak aktif")
//     }
// }

enum Cuaca {
    Cerah,
    Hujan,
    Berawan,
}

fn cek_cuaca(cuaca: Cuaca){
    match cuaca {
        Cuaca::Cerah => println!("Hari ini cerah, ayo keluar!"),
        Cuaca::Hujan => println!("Bawa payung ya, hujan!"),
        Cuaca::Berawan => println!("Awan mendung, tapi belum tentu hujan."),
    }
}


fn main(){
    // variable();
    // tipe_data();
    // konversi_type();
    // casting();
    // data_mahasiswa();
    // cek_nilai_if(77);
    // cek_nilai_if(66);
    // cek_nilai_if(88);
    // cek_nilai_if(99);

    // cek_nilai_switch(66);
    // cek_nilai_switch(77);
    // cek_nilai_switch(88);
    // cek_nilai_switch(99);

    // kategori_usia(11);
    // kategori_usia(16);
    // kategori_usia(58);
    // kategori_usia(70);

    // cetak_ganjil(20);
    // mundur_angka(10);
    // looping_nama(4);
    // tampilkan_nilai()
    // keranjang_belanja()
    // let perkalian = kali_angka(2, 9);
    // println!("{}", perkalian)
    // let lulus: (bool) = cek_kelulusan(80);
    // println!("Status kelulusan: {}", lulus); // true
    // let informasi_mahasiswa = profil_mahasiswa("alfian", 21, 3.7);
    // println!("Nama: {}", informasi_mahasiswa.0);
    // println!("Umur: {}", informasi_mahasiswa.1);
    // println!("ipk: {}", informasi_mahasiswa.2);
    // let mahasiswa1 = Mahasiswa12 {
    //     nama: String::from("Alfian"),
    //     umur: 21,
    //     ipk: 3.9,
    // };

    // println!("Nama : {}", mahasiswa1.nama);
    // println!("Umur : {}", mahasiswa1.umur);
    // println!("ipk : {}", mahasiswa1.ipk);

    // let produk1 = Produk {
    //     nama: String::from("Sosis"),
    //     harga: 5000,
    //     stok: 5,
    // };
    // let produk2 = Produk {
    //     nama: String::from("Cutterpilar"),
    //     harga: 5000,
    //     stok: 4,
    // };
    // tampilkan_produk(&produk1);
    // tampilkan_produk(&produk2);

    // let mahasiswa1 = Mahasiswa {
    //     nama: "Alfian".to_string(),
    //     ipk: 3.5,
    //     aktif: false,
    // };

    // mahasiswa1.perkenalan_mahasiswa();
    // mahasiswa1.status();

    // let mut my_produk = Produk {
    //     nama: "Bronies".to_string(),
    //     harga: 5000,
    //     stok: 54,
    // };
    // my_produk.tampilkan();
    // my_produk.tambah_stok(20);
    // my_produk.diskon(20);

    // let nama = "Alfian".to_string();
    // println!("{}", nama);
    // let nama_baru = nama;
    // println!("{}", nama_baru)

    // let judul_buku = String::from("Belajar Rust untuk Pemula");
    // println!("{}", judul_buku);
    // let judul_favorit = judul_buku;
    // println!("{}", judul_favorit);

    // let bio = String::from("Saya sedang belajar Rust.");
    // cetak_bio(&bio);
    // println!("{}", bio);

    // let mut aktif = false;
    // ubah_status_mahasiswa(&mut aktif);
    // println!("Status Mahasiswa : {}", aktif)

    // let angka = [10, 20, 30, 40, 50];
    // let slice_angka = &angka[1..4];
    // println!("Isi Slice Angka : {:?}", slice_angka);

    // let kalimat = String::from("Saya sedang belajar Rust");
    // let bagian = &kalimat[5..11];
    // println!("Bagian Kalimat : {}", bagian);

    // let nama_lengkap: String = String::from("Alfian Rahman");
    // ambil_nama_depan(nama_lengkap);

    // let status1 = Status::Aktif;
    // let status2 = Status::TidakAktif;

    // cek_status(status1);
    // cek_status(status2);

    let cuaca_hujan = Cuaca::Hujan;
    let cuaca_cerah = Cuaca::Cerah;
    let cuaca_berawan = Cuaca::Berawan;

    cek_cuaca(cuaca_cerah);
    cek_cuaca(cuaca_hujan);
    cek_cuaca(cuaca_berawan);

}