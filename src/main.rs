use std::collections::HashMap;

struct Person<'a>{
    name: &'a str,
    location: &'a str,
}

fn get_city<'a,'b>(code: &'a str, airport_codes: &'b HashMap<&str, &str>) -> &'b str {
    airport_codes
        .get(code)
        .expect("We don't know this location!")
}
fn main() {

    let me = Person{
        name: "Charles",
        location:"JKIA",
    };




    let airport_codes = HashMap::from([
        ("DAR", "Dar-Esalaam"),
        ("JKIA", "Nairobi"),
        ("MOM", "Mumbai"),
    ]);


    println!(
        "Welcome, {} from {}!",
        me.name,
        get_city(&me.location, &airport_codes)
    );

}
