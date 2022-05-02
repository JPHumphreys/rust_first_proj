fn basic(){
    let animal = "Duck";
    match animal{
        "Duck" => print!("Quack"),
        "Dog" => print!("Woof"),
        _ => print!("Beep Boop")
    }


    let frequency = 384;

    match frequency{
        200..=500 => {
            print!("valid");
        }
        _ => {
            print!("invalid");
        }
    }



    if let animal = "Duck"{
        print!("Quack");
    }
}