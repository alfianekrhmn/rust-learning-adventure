mod variable;
mod data_types;
mod functions;

fn cek_suhu(suhu: i32) -> &'static str {
    if suhu > 30 {
        "Panas"
    }else if suhu >= 20 && suhu <= 30 {
        "Nyaman"
    }else{
        "Dingin"
    }
}

fn status_kode(http_code: u16) {
    match http_code {
        200 => println!("OK"),
        404 => println!("Not Found"),
        500 => println!("Server Error"),
        _ => println!("Kode tidak dikenali"),
    }
}

fn main() {
    // variable::run();
    // data_types::run();
    // functions::run();
    // let mut suhu_hari_ini = 39;
    // let pengecekan_suhu = cek_suhu(suhu_hari_ini);
    // println!("Suhu hari ini adalah {} dan itu sangat {} untuk saya", suhu_hari_ini, pengecekan_suhu);
    // suhu_hari_ini = 29;
    // let pengecekan_suhu = cek_suhu(suhu_hari_ini);
    // println!("Suhu hari ini adalah {} dan itu sangat {} untuk saya", suhu_hari_ini, pengecekan_suhu);
    // suhu_hari_ini = 19;
    // let pengecekan_suhu = cek_suhu(suhu_hari_ini);
    // println!("Suhu hari ini adalah {} dan itu sangat {} untuk saya", suhu_hari_ini, pengecekan_suhu);


    let mut status = status_kode(200);
    let mut status = status_kode(404);
    let mut status = status_kode(500);
    let mut status = status_kode(600);
}
