use crate::entities::Animal;
use crate::entities::Drink;
use crate::entities::Food;
use crate::entities::Fruit;
use crate::entities::Transport;

pub fn drink_icon(drink: Drink) -> String{
    let icon: String;
    match drink {
        Drink::Beer => icon=String::from("🍺"),
        Drink::Coffee => icon=String::from("☕"),
        Drink::Milk => icon=String::from("🥛"),
        Drink::Tea => icon=String::from("🫖"),
        Drink::Water => icon=String::from("🚰"),
        Drink::Wine => icon=String::from("🍷"),
        Drink::Any => icon=String::from("🍺☕🥛🫖🚰🍷") 
    }
    icon
}

pub fn fruit_icon(fruit: Fruit) -> String{
    let icon: String;
    match fruit {
        Fruit::Apple => icon=String::from("🍎"),
        Fruit::Orange => icon=String::from("🍊"),
        Fruit::Banana => icon=String::from("🍌"),
        Fruit::Grape => icon=String::from("🍇"),
        Fruit::Strawberry => icon=String::from("🍓"),
        Fruit::Pear => icon=String::from("🍐"),
        Fruit::WaterMelon => icon=String::from("🍉"),
        Fruit::Cherry => icon=String::from("🍒"),
        Fruit::Any => icon=String::from("🍎🍊🍌🍇🍓🍐🍉🍒") 
    }
    icon
}

pub fn animal_icon(animal: Animal) -> String{
    let icon: String;
    match animal {
        Animal::Bird => icon=String::from("🐦"),
        Animal::Cat => icon=String::from("😺"),
        Animal::Dog => icon=String::from("🐶"),
        Animal::Fish => icon=String::from("🐟"),
        Animal::Horse => icon=String::from("🐎"),
        Animal::Rabbit => icon=String::from("🐇"),
        Animal::Pig => icon=String::from("🐖"),
        Animal::Any => icon=String::from("🐦😺🐶🐟🐎🐖🐇")
    }
    icon
}

pub fn food_icon(food: Food) -> String{
    let icon: String;
    match food {
        Food::Bread => icon=String::from("🍞"),
        Food::Croissant => icon=String::from("🥐"),
        Food::Cake => icon=String::from("🍰"),
        Food::Pizza => icon=String::from("🍕"),
        Food::Rice => icon=String::from("🍚"),
        Food::Soup => icon=String::from("🥣"),
        Food::Any => icon=String::from("🍞🥐🍰🍕🍚🥣") 
    }
    icon
}

pub fn transport_icon(transport: Transport) -> String{
    let icon: String;
    match transport {
        Transport::Ambulance => icon=String::from("🚑"),
        Transport::Bus => icon=String::from("🚌"),
        Transport::Car => icon=String::from("🚗"),
        Transport::Taxi => icon=String::from("🚖"),
        Transport::Any => icon=String::from("🚑🚌🚗🚖")
    }
    icon
}
