mod below100;
use below100::below100;

pub fn below1000(number: u32) -> String {
    let mut result=String::new();
    let diff: u32;
    if number < 100 {
        result.push_str(&below100(number));
    } else if number == 100 {
        result.push_str("cien");
    } else if number < 200 {
        result.push_str("ciento ");
        diff = number - 100;
        result.push_str(&below100(diff));
    } else if number == 200 {
        result.push_str("doscientos");
    } else if number < 300 {
        result.push_str("doscientos ");
        diff = number - 200;
        result.push_str(&below100(diff));
    } else if number == 300 {
        result.push_str("trescientos");
    } else if number < 400 {
        result.push_str("trescientos ");
        diff = number - 300;
        result.push_str(&below100(diff));
    } else if number == 400 {
        result.push_str("cuatrocientos");
    } else if number < 500 {
        result.push_str("cuatrocientos ");
        diff = number - 400;
        result.push_str(&below100(diff));
    } else if number == 500 {
        result.push_str("quinientos");
    } else if number < 600 {
        result.push_str("quinientos ");
        diff = number - 500;
        result.push_str(&below100(diff));
    } else if number == 600 {
        result.push_str("seiscientos");
    } else if number < 700 {
        result.push_str("seiscientos ");
        diff = number - 600;
        result.push_str(&below100(diff));
    } else if number == 700 {
        result.push_str("setecientos");
    } else if number < 800 {
        result.push_str("setecientos ");
        diff = number - 700;
        result.push_str(&below100(diff));
    } else if number == 800 {
        result.push_str("ochocientos");    
    } else if number < 900 {
        result.push_str("ochocientos ");
        diff = number - 800;
        result.push_str(&below100(diff));
    } else if number == 900 {
        result.push_str("novecientos");
    } else if number < 1000 {
        result.push_str("novecientos ");
        diff = number - 900;
        result.push_str(&below100(diff));
    }
    result
}
