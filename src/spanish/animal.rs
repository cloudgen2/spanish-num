use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Animal;

pub fn to_thing<'a>(num: u32, animal: Animal) -> Thing<'a> {
    let mut result: Thing;
    match animal {
        Animal::Bird => result = Thing::new( Sex::Male, false, "pájaro", "pájaros"),
        Animal::Cat => result = Thing::new( Sex::Male, false, "gato", "gatos"),
        Animal::Dog => result = Thing::new( Sex::Male, false, "perro", "perros"),
        Animal::Fish => result = Thing::new( Sex::Male, false, "pez", "peces"),
        Animal::Horse => result = Thing::new( Sex::Male, false, "caballo", "caballos"),
        Animal::Rabbit => result = Thing::new( Sex::Male, false, "conejo", "conejos"),
        Animal::Pig => result = Thing::new( Sex::Male, false, "cerdo", "cerdos"),
        Animal::Any => result = Thing::new( Sex::Male, true, "animal","animales")
    }
    result.set_num(num);
    result 
}
