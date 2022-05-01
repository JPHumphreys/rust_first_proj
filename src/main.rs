#![allow(unused_variables)]

fn main() {
    print_lat_long();
    print_strings();
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
}
