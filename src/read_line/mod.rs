extern crate regex;
use regex::Regex;
use std::io::{self, Write};


pub fn read_line() -> String {
    let mut ans = String::new();
    loop {
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut ans) {
            Ok(_) => {
                let processed_line = process_line(&ans);
                if !processed_line.is_empty() {
                    return processed_line;
                }
            }
            Err(_) => {
                ans = String::new();
                return ans;
            }
        }
    }
}

fn process_line(input:&str) -> String{
    let re = Regex::new(r"[\s]+").unwrap();
    let re2 = Regex::new(r"[^A-Za-z0-9ÁáÀàÄäÂâÇçÉéÈèÊêËëÌìÍíÎîÏïÑñÒòÓóÖöÔôŒœẞßÜüÙùÚúÛûŸÿ\.,¿\?¡!\-\s']+").unwrap();
    let re3 = Regex::new(r"\bhundert").unwrap();
    let ans01=re2.replace_all(input.trim(),"");
    let ans02=re.replace_all(&ans01," ");
    let ans03=re3.replace_all(&ans02,"einhundert");
    ans03.trim().to_string()
}