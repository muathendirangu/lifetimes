struct Person<'a>{
    name:String,
    location:&'a String,
}

fn main() {
    let tz = String::from("Tanzania");
    let me = Person{
        name:String::from("Charles"),
        location:&tz,
    };

    let brad = Person{
        name:String::from("Bradley"),
        location:&tz,
    };


    println!("Hello, {} from {}",me.name,me.location);
    println!("Hello mr {}, how is {}",brad.name, brad.location)
}
