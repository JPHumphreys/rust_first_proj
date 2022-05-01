fn main() {
    print_lat_long();
    print_strings();
}

fn print_lat_long(){
    let location = ("location1", 41.407, -81.2);
    let (name, latitude, longitude) = location;
    println!("Location name: {}, lat: {}, lon: {}",
     name, latitude, longitude);
}

fn print_strings(){

}
