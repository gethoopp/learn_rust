
#[tokio::main]
async fn main() {
    // Jalankan server dari routes
    learn_rust::run_server().await;
}


//curl -X POST 'http://localhost:8000/register' -H 'Content-Type: application/json' -d '{ "user_id": 2 }'
//curl -X DELETE 'http://localhost:8000/register/5113c28c9d0d4bfd9d87466a4dca258e'
//websocat ws://127.0.0.1:3030/ws/1234



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


    let number_two: i16 = 1000;
    println!("{}", number_two);

    
    let number_twohalf: i8 = i8::try_from(number_two).unwrap();
    println!("ini hasil nomor {}", number_twohalf);


    let number_two: f32 = 0.02;
    println!("{}", number_two)
}


//konsep numeric operator
#[test]
fn numeric_operator() {}


//konsep augmented assignment
#[test]
fn augmented_assignment() {}