#[derive(Debug)]
#[allow(dead_code)]
enum Flavour{
    Orange,
    Guava,
    Mango,
    Apple,
    Passion,
    Coke,
    Sprite,
}

#[allow(dead_code)]
struct Drink{
    capacity: i32,
    flavour: Flavour,
    is_fizzy: bool,
}

fn check_if_fizzy(drink: &Drink) -> bool {
    match drink.flavour {
        Flavour::Coke | Flavour::Sprite => true,
        _ => false,
    }
}

fn main() {
    let my_drink=Drink{
        capacity: 500,
        flavour: Flavour::Coke,
        is_fizzy: true,
    };

    let is_fizzy = check_if_fizzy(&my_drink);  

    println!("Your drink is a {:?} drink, Is the drink fizzy? {}", my_drink.flavour,is_fizzy); 
}
