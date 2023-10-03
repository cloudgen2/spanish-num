use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Food;

pub fn to_thing<'a>(num: u32, food: Food) -> Thing<'a> {
    let mut result: Thing;
    match food {
        Food::Bread => result = Thing::new( Sex::Male, false, "pedazo de pan", "pedazos de pan"),
        Food::Croissant => result = Thing::new( Sex::Male, false, "croissant", "croissants"),
        Food::Cake => result = Thing::new( Sex::Male, false, "pedazo de pastel", "pedazos de pastel"),
        Food::Pizza => result = Thing::new( Sex::Female, false, "rebanada de pizza", "rebanadas de pizza"),
        Food::Rice => result = Thing::new( Sex::Male, false, "cuenco de arroz", "cuencos de arroz"),
        Food::Soup => result = Thing::new( Sex::Male, false, "plato de sopa", "platos de sopa"),
        Food::Any => result = Thing::new( Sex::Male, false, "alimento","alimentos")
    }
    result.set_num(num);
    result 
}
