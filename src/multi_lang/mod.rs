mod spanish;
use rand::Rng;
use crate::entities::Animal;
use crate::entities::Drink;
use crate::entities::Food;
use crate::entities::Fruit;
use crate::entities::Transport;
use spanish::ask_n_check_drink_sub;
use spanish::ask_n_check_fruit_sub;
use spanish::ask_n_check_food_sub;
use spanish::ask_n_check_animal_sub;
use spanish::ask_n_check_transport_sub;
use spanish::ask_n_check_num_sub;
use spanish::print_level_sub;
use crate::spanish::print_update_at_sub;
use crate::spanish::print_type_exit_to_exit_sub;

#[derive(Copy, Clone)]
pub enum Lang {
    English,
    French,
    German,
    Spanish
}

pub fn print_type_exit_to_exit( lang: Lang) {
    match lang {
        Lang::English => print_type_exit_to_exit_sub(),
        Lang::French => print_type_exit_to_exit_sub(),
        Lang::German => print_type_exit_to_exit_sub(),
        Lang::Spanish => print_type_exit_to_exit_sub()
    }
}

pub fn print_update_at(date: &str, lang: Lang){
    match lang {
        Lang::English => print_update_at_sub(date),
        Lang::French => print_update_at_sub(date),
        Lang::German => print_update_at_sub(date),
        Lang::Spanish => print_update_at_sub(date)
    }
}

pub fn print_level(level: u32, lang: Lang){
    match lang {
        Lang::English => print_level_sub(level),
        Lang::French => print_level_sub(level),
        Lang::German => print_level_sub(level),
        Lang::Spanish => print_level_sub(level)
    }
}

pub fn ask_n_check_drink(q: u32, min: u32, max: u32, drink_type:Drink, lang:Lang) {
    let mut drink_val:Drink = drink_type;
    if matches!(drink_val, Drink::Any) {
        let r = rand::thread_rng().gen_range(0..=4);
        match r{
            0 => drink_val = Drink::Beer,
            1 => drink_val = Drink::Coffee,
            2 => drink_val = Drink::Milk,
            3 => drink_val = Drink::Tea,
            4 => drink_val = Drink::Wine,
            _ => drink_val = Drink::Any
        }
    }
    let num: u32 = rand::thread_rng().gen_range(min..max);
    
    match lang  {
        Lang::English => ask_n_check_drink_sub(q, num, drink_val),
        Lang::French => ask_n_check_drink_sub(q, num, drink_val),
        Lang::German => ask_n_check_drink_sub(q, num, drink_val),
        Lang::Spanish => ask_n_check_drink_sub(q, num, drink_val)
    }
}

pub fn ask_n_check_fruit(q: u32, min: u32, max: u32, fruit_type:Fruit, lang:Lang) {
    let mut fruit_val:Fruit = fruit_type;
    if matches!(fruit_val, Fruit::Any) {
        let r = rand::thread_rng().gen_range(0..=6);
        match r{
            0 => fruit_val = Fruit::Apple,
            1 => fruit_val = Fruit::Orange,
            2 => fruit_val = Fruit::Banana,
            3 => fruit_val = Fruit::Strawberry,
            4 => fruit_val = Fruit::Pear,
            5 => fruit_val = Fruit::Cherry,
            6 => fruit_val = Fruit::WaterMelon,
            _ => fruit_val = Fruit::Any
        }
    }  
    let num: u32 = rand::thread_rng().gen_range(min..max);
    match lang  {
        Lang::English => ask_n_check_fruit_sub(q, num, fruit_val),
        Lang::French => ask_n_check_fruit_sub(q, num, fruit_val),
        Lang::German => ask_n_check_fruit_sub(q, num, fruit_val),
        Lang::Spanish => ask_n_check_fruit_sub(q, num, fruit_val)
    }
}

pub fn ask_n_check_food(q: u32, min: u32, max: u32, food_type:Food, lang:Lang) {
    let mut food_val:Food = food_type;
    if matches!(food_val, Food::Any) {
        let r = rand::thread_rng().gen_range(0..=5);
        match r{
            0 => food_val = Food::Bread,
            1 => food_val = Food::Croissant,
            2 => food_val = Food::Cake,
            3 => food_val = Food::Pizza,
            4 => food_val = Food::Rice,
            5 => food_val = Food::Soup,
            _ => food_val = Food::Any
        }
    }
    let num: u32 = rand::thread_rng().gen_range(min..max);
    match lang  {
        Lang::English => ask_n_check_food_sub(q, num, food_val),
        Lang::French => ask_n_check_food_sub(q, num, food_val),
        Lang::German => ask_n_check_food_sub(q, num, food_val),
        Lang::Spanish => ask_n_check_food_sub(q, num, food_val)
    }
}

pub fn ask_n_check_animal(q: u32, min: u32, max: u32, animal_type:Animal, lang:Lang) {
    let mut animal_val:Animal = animal_type;
    if matches!(animal_val, Animal::Any) {
        let r = rand::thread_rng().gen_range(0..=5);
        match r{
            0 => animal_val = Animal::Bird,
            1 => animal_val = Animal::Cat,
            2 => animal_val = Animal::Dog,
            3 => animal_val = Animal::Fish,
            4 => animal_val = Animal::Horse,
            5 => animal_val = Animal::Rabbit,
            _ => animal_val = Animal::Any
        }
    }
    let num: u32 = rand::thread_rng().gen_range(min..max);
    match lang  {
        Lang::English => ask_n_check_animal_sub(q, num,  animal_val),
        Lang::French => ask_n_check_animal_sub(q, num,  animal_val),
        Lang::German => ask_n_check_animal_sub(q, num,  animal_val),
        Lang::Spanish => ask_n_check_animal_sub(q, num,  animal_val)
    }
}

pub fn ask_n_check_transport(q: u32, min: u32, max: u32, transport_type:Transport, lang:Lang) {
    let mut transport_val:Transport = transport_type;
    if matches!(transport_val, Transport::Any) {
        let r = rand::thread_rng().gen_range(0..=1);
        match r{
            0 => transport_val = Transport::Bus,
            1 => transport_val = Transport::Car,
            _ => transport_val = Transport::Any
        }
    }
    let num: u32 = rand::thread_rng().gen_range(min..max);
    match lang  {
        Lang::English => ask_n_check_transport_sub(q, num,  transport_val),
        Lang::French => ask_n_check_transport_sub(q, num,  transport_val),
        Lang::German => ask_n_check_transport_sub(q, num,  transport_val),
        Lang::Spanish => ask_n_check_transport_sub(q, num,  transport_val)
    }
}

pub fn ask_n_check_num(q: u32, min: u32, max: u32, lang:Lang) {
    let num: u32 = rand::thread_rng().gen_range(min..max);
    match lang  {
        Lang::English => ask_n_check_num_sub(q, num),
        Lang::French => ask_n_check_num_sub(q, num),
        Lang::German => ask_n_check_num_sub(q, num),
        Lang::Spanish => ask_n_check_num_sub(q, num)
    }
}

pub fn ask_n_check_all(q: u32, min: u32, max: u32, lang:Lang) {
    let n = rand::thread_rng().gen_range(0..=3);
    if n % 4 == 0 {
        ask_n_check_num(q, min, max, lang);
    } else if n % 4 == 1 {
        ask_n_check_fruit(q, min, max, Fruit::Any, lang);
    } else if n % 4 == 2 {
        ask_n_check_animal(q, min, max, Animal::Any, lang);
    } else if n % 4 == 3 {
        ask_n_check_drink(q, min, max, Drink::Any, lang);
    } 
}