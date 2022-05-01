fn main() {
    let location = ("location1", 41.407, -81.2);
    let (name, latitude, longitude) = location;
    println!("Location name: {}, lat: {}, lon: {}",
     name, latitude, longitude);
}
