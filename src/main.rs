
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
    let mut num = 18;
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