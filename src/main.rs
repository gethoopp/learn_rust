
use learn_rust::print_text;
fn main() {
    print!("Hello, ");
    print_text();
    print!("Welcome to rust");
}


#[test]
fn variabel(){
    //variabel di bawah bertipe immutable tidak bisa diubah nilainya dari inisialisasi awal
    let  name = "Haliim pamungkas harjo suyono";
    println!("Haloo {}",name);

    //variabel di bawah ini bertipe muttable bisa diubah nilainya dari inisisalisasi awal
    let  mut num = 18;
    let err = "Umur kamu kurang dari 25";
    println!("Umur saya {}", num);

    if num <= 20 {
        println!("{}",err);
         println!("Umur kamu baru {}",num);   
    } else {
        num = num;
        println!("Umur kamu {}",num);  
    }

    
}

//konsep shadowing
#[test]
fn shadowing() {
    let number_phone = 123495;
    println!("Hello {}", number_phone);

    let number_phone = "12349219";
    println!("Hello {}",number_phone);


    println!("Ini isi variabel name {}", number_phone);
}

//konsep number tipe data
#[test]
fn number() {
    let number_one: i32 = 1000000;
    println!("{}", number_one);


let number_onehalf: i8 = i8::try_from(number_one).unwrap();
    println!("{}", number_onehalf);


    let number_two: f32 = 0.02;
    println!("{}", number_two)
}


//konsep numeric operator
#[test]
fn numeric_operator() {}


//konsep augmented assignment
#[test]
fn augmented_assignment() {}