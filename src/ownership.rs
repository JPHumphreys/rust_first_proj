fn ownership(){
    let mut original = String::from("original value");
    println!("{}", original);
    {
        let next = &mut original;
        *next = String::from("next value");

        println!("{}", next);
        println!("{}", original);
    }

    println!("{}", original);
}