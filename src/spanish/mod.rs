extern crate regex;

pub mod number;
mod animal;
mod drink;
mod food;
mod fruit;
mod transport;

use crate::entities::Animal;
use crate::entities::Food;
use crate::entities::Fruit;
use crate::entities::Drink;
use crate::entities::Transport;
use crate::entities::Sex;
use crate::thisis::ThisIs;
use crate::multi_lang::Lang;

use colored::*;
use regex::Regex;
use number::all_num;

pub struct Thing<'a> {
    sex: Sex,
    num: u32,
    starts_with_vowel: bool,
    single: &'a str,
    plural: &'a str,
    lang: Lang
}

impl Thing<'_> {
    pub fn new<'a> (sex: Sex, starts_with_vowel: bool, single: &'a str, plural: &'a str) -> Thing <'a>{
        Thing {
            sex: sex,
            starts_with_vowel: starts_with_vowel,
            num: 0,
            single: single, 
            plural: plural,
            lang: Lang::Spanish
        }
    }
}

impl ThisIs for Thing<'_>  {
    fn number_of(&self) -> String {
        let mut result: String;
        match self.lang {
            Lang::English => {
                if self.num == 1 {
                    if self.starts_with_vowel {
                        result=String::from("an ")
                    } else {
                        result=String::from("a ")
                    }
                    result.push_str(self.single);
                } else {
                    result=String::from(all_num(self.num));
                    result.push_str(" ");
                    result.push_str(self.plural)
                } 

            },
            Lang::French => {
                if self.num == 1 {
                    match self.sex {
                        Sex::Any => result=String::from("un "),
                        Sex::Male => result=String::from("un "),
                        Sex::Female => result=String::from("une ")
                    }
                    result.push_str(self.single);
                } else {
                    result=String::from(all_num(self.num));
                    result.push_str(" ");
                    result.push_str(self.plural)
                } 
            },
            Lang::German => {
                if self.num == 1 {
                    match self.sex {
                        Sex::Any => result=String::from("ein "),
                        Sex::Male => result=String::from("ein "),
                        Sex::Female => result=String::from("eine ")
                    }
                    result.push_str(self.single);
                } else {
                    result=String::from(all_num(self.num));
                    result.push_str(" ");
                    result.push_str(self.plural)
                }
            },
            Lang::Spanish => {
                if self.num == 1 {
                    match self.sex {
                        Sex::Any => result=String::from("un "),
                        Sex::Male => result=String::from("un "),
                        Sex::Female => result=String::from("una ")
                    }
                    result.push_str(self.single);
                } else {
                    result=String::from(all_num(self.num));
                    result.push_str(" ");
                    result.push_str(self.plural)
                }
            }
        }
        result
    }
    fn this_is(&self) -> String {
        let mut result:String;
        match self.lang {
            Lang::English => {
                if self.num == 1 {
                    result=String::from("This is ");
                } else {
                    result=String::from("These are ");
                }        
            },
            Lang::French => {
                if self.num == 1 {
                    result=String::from("Ceci est ");
                } else {
                    result=String::from("Ce sont ");
                }
            },
            Lang::German => {
                if self.num == 1 {
                    result=String::from("Das ist ");
                } else {
                    result=String::from("Das sind ");
                }
            },
            Lang::Spanish => {
                if self.num == 1 {
                    match self.sex {
                        Sex::Any => result=String::from("Este es "),
                        Sex::Male => result=String::from("Este es "),
                        Sex::Female => result=String::from("Esta es ")
                    }
                } else {
                    match self.sex {
                        Sex::Any => result=String::from("Estos son "),
                        Sex::Male => result=String::from("Estos son "),
                        Sex::Female => result=String::from("Estas son ")
                    }
                }
            },
        }
        result.push_str(& self.number_of());
        result.push_str(".");
        result
    }
    fn set_num(&mut self, num:u32) {
        self.num = num;
    }
}

pub fn print_update_at_sub(date: &str){
    println!("# Actualizado el: {}", date);
}

pub fn print_type_exit_to_exit_sub() {
    println!("¡Escriba '{}' para salir!", "exit".red());
    println!("");
}

pub fn fruit<'a>(num: u32, fruit: Fruit) -> Thing<'a> {
    fruit::to_thing(num, fruit)
}

pub fn drink<'a>(num: u32, drink: Drink) -> Thing<'a> {
    drink::to_thing(num, drink)
}

pub fn animal<'a>(num: u32, animal: Animal) -> Thing<'a> {
    animal::to_thing(num, animal)
}

pub fn food<'a>(num: u32, food: Food) -> Thing<'a> {
    food::to_thing(num, food)
}

pub fn transport<'a>(num: u32, transport: Transport) -> Thing<'a> {
    transport::to_thing(num, transport)
}

pub fn print_level(level: u32) {
    if level == 1 {
        println!("== Primera etapa L1 ==");
    } else {
        println!("== Nivel {} L{} ==", level, level);
    }
}

pub fn print_what_is_it(question: u32, quantity: u32, icon: &String, highlight: &str) {
    print!("{}{} {}{}{} ",
        highlight.red(), 
        format!("{})",question).blue().bold(), 
        "¿Qué es? ( ".yellow(),
        format!("{}{}", quantity, icon),
        " )?".yellow());
}

pub fn print_what_num(question: u32, quantity: u32, highlight: &str) {
    print!("{}{} {}{}{} ",
        highlight.red(), 
        format!("{})",question).blue().bold(), 
        format!("¿Cuál es el número ").yellow(), 
        quantity, 
        format!(" en español?").yellow());
}

pub fn print_correct() {
    println!("» {}","¡Es correcto!".dimmed());
}

pub fn print_correct_ans(ans: &String) {
    println!("» {}{}{}","La respuesta correcta es '".dimmed(), ans.italic(),"'.".dimmed());
}

pub fn process_ans(input:&str) -> String{
    let re = Regex::new(r"\s*[\.,/!:;\?\s]+\s*").unwrap();
    let ans01=re.replace_all(input.trim()," ");
    ans01.trim().to_lowercase().to_string()
}