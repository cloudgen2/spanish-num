pub fn below100(number: u32) -> String {
    let numbers = vec![
        "cero",
        "uno",
        "dos",
        "tres",
        "cuatro",   
        "cinco",
        "seis",
        "siete",
        "ocho",
        "nueve",
        "diez",
        "once",
        "doce",
        "trece",
        "catorce",
        "quince",
        "dieciséis",
        "diecisiete",
        "dieciocho",
        "diecinueve",
        "veinte", 
        "veintiuno",
        "veintidós",
        "veintitrés",
        "veinticuatro",
        "veinticinco",
        "veintiséis"];
    let mut result: String;
    let diff: u32;
    if number < 27 {
        result=String::from(numbers[number as usize]);
    } else if number < 30 {
        result=String::from("veinti");
        diff = number - 20;
        result.push_str(numbers[diff as usize]);
    } else if number < 40 {
        result=String::from("treinta");
        diff = number - 30;
        if diff > 0 {
            result.push_str(" y ");
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 50 {
        result=String::from("cuarenta");
        diff = number - 40;
        if diff > 0 {
            result.push_str(" y ");
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 60 {
        result=String::from("cincuenta");
        diff = number - 50;
        if diff > 0 {
            result.push_str(" y ");
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 70 {
        result=String::from("sesenta");
        diff = number - 60;
        if diff > 0 {
            result.push_str(" y ");
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 80 {
        result=String::from("setenta");
        diff = number - 70;
        if diff > 0 {
            result.push_str(" y ");
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 90 {
        result=String::from("ochenta");
        diff = number - 80;
        if diff > 0 {
            result.push_str(" y ");
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 100 {
        result=String::from("noventa");
        diff = number - 90;
        if diff > 0 {
            result.push_str(" y ");
            result.push_str(numbers[diff as usize]);
        }
    } else {
        result = String::new();
    }
    result
}