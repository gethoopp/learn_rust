
use learn_rust::print_text;
fn main() {
    print!("Hello, ");
    print_text();
    print!("Welcome to rust");
}


#[test]
fn variabel(){
    let  name = "Haliim pamungkas harjo suyono";
    println!("Haloo {}",name);


    let mut num = 20;
    println!("Umur saya {}", num);

    if(num == 20) {
        num = 10;
    }

    println!("Umur baru saya {}",num);    
}