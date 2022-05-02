fn basic(){
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10{
            break;
        }
    }

    while counter <= 10{
        counter += 10;
        println!("hello");
    }

    for index in 1..11{
        println!("1-10");
    }

    for index in 1..=10{
        print!("1-10");
    }

    let aircrafts = ["1","2","3"];

    for aircraft in aircrafts.iter(){
        println!(aircraft);
    }
}