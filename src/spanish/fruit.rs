use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Fruit;

pub fn to_thing<'a>(num: u32, fruit: Fruit) -> Thing<'a> {
    let mut result: Thing;
    match fruit {
        Fruit::Apple => result = Thing::new( Sex::Female, false, "manzana","manzanas"),
        Fruit::Orange => result = Thing::new( Sex::Female, false, "naranja","naranjas"),
        Fruit::Banana => result = Thing::new( Sex::Male, false, "plátano","plátanos"),
        Fruit::Strawberry => result = Thing::new( Sex::Female, false, "fresa","fresas"),
        Fruit::Pear => result = Thing::new( Sex::Female, false, "pera", "peras" ),
        Fruit::WaterMelon => result = Thing::new( Sex::Female, false, "sandía", "sandías" ),
        Fruit::Cherry => result = Thing::new( Sex::Female, false, "cereza", "cerezas" ),
        Fruit::Any => result = Thing::new( Sex::Female, false, "fruta", "frutas")
    }
    result.set_num(num);
    result 
}