use crate::entities::Animal;
use crate::entities::Drink;
use crate::entities::Food;
use crate::entities::Fruit;
use crate::entities::Transport;
use crate::multi_lang::print_level;
use crate::multi_lang::ask_n_check_drink;
use crate::multi_lang::ask_n_check_food;
use crate::multi_lang::ask_n_check_fruit;
use crate::multi_lang::ask_n_check_animal;
use crate::multi_lang::ask_n_check_transport;
use crate::multi_lang::ask_n_check_num;
use crate::multi_lang::ask_n_check_all;
use crate::multi_lang::Lang;

pub fn function(lang:Lang) {
    print_level(1, lang);
    for n in 1..11 {
        ask_n_check_num(n, 1 , 11, lang);
    }
    ask_n_check_fruit(12, 1, 2, Fruit::Apple, lang);
    for n in 13..16 {
        ask_n_check_fruit(n, 1, 11, Fruit::Apple, lang);
    }
    ask_n_check_fruit(17, 1, 2, Fruit::Orange, lang);
    for n in 18..21 {
        ask_n_check_fruit(n, 1, 11, Fruit::Orange, lang);
    }
    print_level(2, lang);
    for n in 21..31 {
        ask_n_check_num(n, 11, 21, lang);
    }
    ask_n_check_animal(31, 1, 2, Animal::Rabbit, lang);
    for n in 32..35 {
        ask_n_check_animal(n, 1, 21, Animal::Rabbit, lang);
    }
    ask_n_check_fruit(36, 1, 2, Fruit::Strawberry, lang);
    for n in 37..41 {
        ask_n_check_fruit(n, 1, 21, Fruit::Strawberry, lang);
    }
    print_level(3, lang);
    for n in 41..56 {
        ask_n_check_num(n, 21, 30, lang);
    }
    ask_n_check_fruit(56, 1, 2, Fruit::Banana, lang);
    for n in 57..61 {
        ask_n_check_fruit(n, 1, 30, Fruit::Banana, lang);
    }
    print_level(4, lang);
    for n in 61..76 {
        ask_n_check_num(n, 30, 40, lang);
    }
    ask_n_check_animal(76, 1, 2, Animal::Cat, lang);
    for n in 77..81 {
        ask_n_check_animal(n, 1, 20, Animal::Cat, lang);
    }
    print_level(5, lang);
    for n in 81..96 {
        ask_n_check_num(n, 40, 50, lang);
    }
    ask_n_check_animal(96, 1, 2, Animal::Dog, lang);
    for n in 97..101 {
        ask_n_check_animal(n, 1, 20, Animal::Dog, lang);
    }
    print_level(6, lang);
    for n in 101..116 {
        ask_n_check_num(n, 50, 60, lang);
    }
    ask_n_check_animal(116, 1, 2, Animal::Horse, lang);
    for n in 117..121 {
        ask_n_check_animal(n, 1, 20, Animal::Horse, lang);
    }
    print_level(7, lang);
    for n in 121..139 {
        ask_n_check_num(n, 60, 70, lang);
    }
    for n in 129..136 {
        ask_n_check_num(n, 70, 80, lang);
    }
    ask_n_check_animal(136, 1, 2, Animal::Bird, lang);
    for n in 136..141 {
        ask_n_check_animal(n, 1, 20, Animal::Bird, lang);
    }
    print_level(8, lang);
    ask_n_check_num(141, 80, 81, lang);
    for n in 142..150 {
        ask_n_check_num(n, 80, 90, lang);
    }
    ask_n_check_num(150, 90, 91, lang);
    for n in 151..157 {
        ask_n_check_num(n, 90, 99, lang);
    }
    ask_n_check_num(157, 100, 101, lang);
    for n in 157..161 {
        ask_n_check_num(n, 100, 199, lang);
    }
    print_level(9, lang);
    ask_n_check_drink(161, 1, 2, Drink::Coffee, lang);
    for n in 162..169 {
        ask_n_check_drink(n, 1, 20, Drink::Coffee, lang);
    }
    ask_n_check_drink(169, 1, 2, Drink::Water, lang);
    for n in 170..176 {
        ask_n_check_drink(n, 1, 20, Drink::Water, lang);
    }
    print_level(10, lang);
    ask_n_check_num(181, 200, 201, lang);
    ask_n_check_num(182, 300, 301, lang);
    for n in 183..186 {
        ask_n_check_num(n, 200, 400, lang);
    }
    ask_n_check_num(186, 400, 401, lang);
    ask_n_check_num(187, 500, 501, lang);
    for n in 188..191 {
        ask_n_check_num(n, 400, 600, lang);
    }
    ask_n_check_drink(191, 1, 2, Drink::Milk, lang);
    for n in 192..196 {
        ask_n_check_drink(n, 1, 20, Drink::Milk, lang);
    }
    ask_n_check_num(196, 600, 601, lang);
    ask_n_check_num(197, 700, 701, lang);
    for n in 198..201 {
        ask_n_check_num(n, 600, 800, lang);
    }
    print_level(11, lang);
    ask_n_check_num(201, 800, 801, lang);
    ask_n_check_num(202, 900, 901, lang);
    for n in 203..209 {
        ask_n_check_num(n, 800, 1000, lang);
    }
    ask_n_check_drink(209, 1, 2, Drink::Beer, lang);
    for n in 210..216 {
        ask_n_check_drink(n, 1, 20, Drink::Beer, lang);
    }
    ask_n_check_drink(216, 1, 2, Drink::Wine, lang);
    for n in 217..221 {
        ask_n_check_drink(n, 1, 20, Drink::Wine, lang);
    }
    print_level(12, lang);
    ask_n_check_food(221, 1, 2, Food::Croissant, lang);
    for n in 222..229 {
        ask_n_check_food(n, 1, 20, Food::Croissant, lang);
    }
    ask_n_check_food(229, 1, 2, Food::Bread, lang);
    for n in 230..236 {
        ask_n_check_food(n, 1, 20, Food::Bread, lang);
    }
    ask_n_check_food(236, 1, 2, Food::Cake, lang);
    for n in 237..241 {
        ask_n_check_food(n, 1, 20, Food::Cake, lang);
    }
    print_level(13, lang);
    ask_n_check_food(241, 1, 2, Food::Pizza, lang);
    for n in 242..249 {
        ask_n_check_food(n, 1, 20, Food::Pizza, lang);
    }
    ask_n_check_food(249, 1, 2, Food::Rice, lang);
    for n in 250..256 {
        ask_n_check_food(n, 1, 20, Food::Rice, lang);
    }
    ask_n_check_food(256, 1, 2, Food::Soup, lang);
    for n in 257..261 {
        ask_n_check_food(n, 1, 20, Food::Soup, lang);
    }
    print_level(14, lang);
    for n in 261..266 {
        ask_n_check_num(n, 1000, 2000, lang);
    }
    ask_n_check_animal(266, 1, 2, Animal::Bird, lang);
    for n in 267..271 {
        ask_n_check_animal(n, 1, 60, Animal::Bird, lang);
    }
    ask_n_check_animal(271, 1, 2, Animal::Fish, lang);
    for n in 272..276 {
        ask_n_check_animal(n, 1, 20, Animal::Fish, lang);
    }
    ask_n_check_fruit(276, 1, 2, Fruit::Pear, lang);
    for n in 277..281 {
        ask_n_check_fruit(n, 1, 20, Fruit::Pear, lang);
    }
    print_level(15, lang);
        ask_n_check_fruit(281, 1, 20, Fruit::Pear, lang);
    for n in 282..286 {
        ask_n_check_fruit(n, 1, 20, Fruit::Pear, lang);
    }
    ask_n_check_fruit(286, 1, 2, Fruit::Cherry, lang);
    for n in 287..296 {
        ask_n_check_fruit(n, 1, 20, Fruit::Cherry, lang);
    }
    ask_n_check_fruit(296, 1, 2, Fruit::WaterMelon, lang);
    for n in 297..301 {
        ask_n_check_fruit(n, 1, 20, Fruit::WaterMelon, lang);
    }
    print_level(16, lang);
    for n in 301..306 {
        ask_n_check_fruit(n, 1, 20, Fruit::WaterMelon, lang);
    }
    for n in 306..314 {
        ask_n_check_fruit(n, 1, 100, Fruit::Any, lang);
    }
    for n in 314..321 {
        ask_n_check_animal(n, 1, 100, Animal::Any, lang);
    }
    print_level(17, lang);
    for n in 321..330 {
        ask_n_check_drink(n, 1, 100, Drink::Any, lang);
    }
    for n in 330..341 {
        ask_n_check_food(n, 1, 100, Food::Any, lang);
    }
    print_level(18, lang);
    ask_n_check_transport(341, 1, 2, Transport::Bus, lang);
    for n in 342..351 {
        ask_n_check_transport(n, 1, 100, Transport::Bus, lang);
    }
    ask_n_check_transport(351, 1, 2, Transport::Car, lang);
    for n in 352..361 {
        ask_n_check_transport(n, 1, 100, Transport::Car, lang);
    }
    print_level(19, lang);
    for n in 361..371 {
        ask_n_check_all(n, 1, 100, lang);
    }
    for n in 371..381 {
        ask_n_check_num(n, 2001, 10000, lang);
    }
}