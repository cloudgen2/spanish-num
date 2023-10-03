mod below1000;
use below1000::below1000;

pub fn all_num(number: u32) -> String {
    let mut result=String::new();
    let diff: u32;
    if number<1000 {
        result.push_str(&below1000( number));
    } else if number == 1000 {
        result.push_str("mil");
    } else if number < 2000 {
        result.push_str("mil ");
        diff = number - 1000;
        result.push_str(&below1000( diff));
    } else if number == 2000 {
        result.push_str("dos mil");
    } else if number < 3000 {
        result.push_str("dos mil ");
        diff = number - 2000;
        result.push_str(&below1000( diff));
    } else if number == 3000 {  
        result.push_str("tres mil");
    } else if number < 4000 {
        result.push_str("tres mil ");
        diff = number - 3000;
        result.push_str(&below1000( diff));
    } else if number == 4000 {
        result.push_str("cuatro mil");
    } else if number < 5000 {
        result.push_str("cuatro mil ");
        diff = number - 4000;
        result.push_str(&below1000( diff));
    } else if number == 5000 {
        result.push_str("cinco mil");
    } else if number < 6000 {
        result.push_str("cinco mil ");
        diff = number - 5000; 
        result.push_str(&below1000( diff));
    } else if number == 6000 {
        result.push_str("seis mil");
    } else if number < 7000 {
        result.push_str("seis mil ");
        diff = number - 6000;
        result.push_str(&below1000( diff));
    } else if number == 7000 {
        result.push_str("siete mil");
    } else if number < 8000 {
        result.push_str("siete mil ");
        diff = number - 7000;
        result.push_str(&below1000( diff));
    } else if number == 8000 {
        result.push_str("ocho mil");
    } else if number < 9000 {
        result.push_str("ocho mil ");
        diff = number - 8000;
        result.push_str(&below1000( diff));
    } else if number == 9000 {
        result.push_str("nueve mil");
    } else if number < 10000 {
        result.push_str("nueve mil ");
        diff = number - 9000;
        result.push_str(&below1000( diff));
    }
    result
}