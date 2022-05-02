#![allow(unused_variables)]

fn main() {
    print_lat_long();
    print_strings();
    create_variables();
    operators();
}

fn print_lat_long(){
    let location = ("location1", 41.407, -81.2);
    let (name, latitude, longitude) = location;
    //println!("Location name: {}, lat: {}, lon: {}", name, latitude, longitude);
}

fn print_strings(){
    let person_name_slice = "Donald Mallard";
    let person_name_string = String::from("Donald Mallard");
    let person_name_slice2 = &person_name_string;
    let person_name_slice3 = person_name_string.as_str();

    let duck = "Duck";
    let airlines = "Airlines";

    let airline_name = [duck, " ", airlines].concat();

    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' ');
    slogan = slogan + "every time";
    print!("{}", slogan);
}

fn create_variables(){
    let my_infered = 0;
    let my_var = 1_i8;
    let my_var2: u32 = 0;
    let _unused_var = 0;

    let my_float = 17.2;
    let my_unsigned:u8 = 5;
    let casted_unsigned = my_unsigned as f32;
    
    let result = my_float / casted_unsigned;
}

fn operators(){
    let squared = i32::pow(8, 2);
    let is_true = 1==1;
    let is_not_true = 1 !=2;

    let have_pass = true;
    let have_id = true;
    let can_board = have_pass && have_id;
}
