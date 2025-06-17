fn main() {
    println!("Hello, Alfian!");
}

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