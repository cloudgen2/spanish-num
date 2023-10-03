use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Drink;

pub fn to_thing<'a>(num: u32, drink: Drink) -> Thing<'a> {
    let mut result: Thing;
    match drink {
        Drink::Beer => result = Thing::new( Sex::Male, false, "vaso de cerveza", "vasos de cerveza"),
        Drink::Coffee => result = Thing::new( Sex::Female, false,"taza de café", "tazas de café"),
        Drink::Milk => result = Thing::new( Sex::Male, false, "vaso de leche", "vasos de leche"),
        Drink::Tea => result = Thing::new( Sex::Female, false, "taza de té", "tazas de té"),
        Drink::Water => result = Thing::new( Sex::Male, false, "vaso de agua", "vasos de agua"),
        Drink::Wine => result = Thing::new( Sex::Female, false, "copa de vino", "copas de vino"),
        Drink::Any => result = Thing::new( Sex::Male, false, "vaso de bebida", "vasos de bebida")
    }
    result.set_num(num);
    result 
}

