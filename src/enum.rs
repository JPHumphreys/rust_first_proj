enum NavigationAids{
    NDB,
    VOR,
    VORDME
}

enum NavigationAids2{
    NDB,//0
    VOR = 4,
    VORDME//7
}

enum NavigationAids3{
    NDB = 2,
    VOR = 4,
    VORDME = 8
}

enum NavigationAids4{
    NDB = 5,
    VOR,//6
    VORDME//7
}

enum NavigationAids5{
    NDB = 5,
    VOR,
    VORDME,
    FIX {name: String, latitude: f32, longitude: f32}
}


fn basic(){
    println!("{}", NavigationAids::NDB as u8);
}

