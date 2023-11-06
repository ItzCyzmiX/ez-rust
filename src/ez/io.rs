

pub fn get_input(msg: &str) -> String {
    use std::io;
    let mut input: String = String::new();

    println!("{msg}");

    io::stdin().read_line(&mut input)
               .expect("Error while reading input!");
    
    return String::from(input.trim());
}

pub struct Colors { }

impl Colors {
    //Foreground Color:
    pub fn fg_red() -> String {
        return "\x1b[31m".to_string();
    }
    pub fn fg_yellow() -> String {
        return "\x1b[33m".to_string();
    }
    pub fn fg_green() -> String {
        return "\x1b[32m".to_string();
    }
    pub fn fg_blue() -> String {
        return "\x1b[34m".to_string();
    }
    pub fn fg_white() -> String {
        return "\x1b[37m".to_string();
    }
    pub fn fg_cyan() -> String {
        return "\x1b[36m".to_string();
    }
    pub fn fg_black() -> String {
        return "\x1b[30m".to_string();
    }
    pub fn fg_magenta() -> String {
        return "\x1b[35m".to_string();
    }
    pub fn fg_reset() -> String {
        return "\x1b[0m".to_string();
    }

    //Background Colors:
    pub fn bg_red() -> String {
        return "\x1b[41m".to_string();
    }
    pub fn bg_yellow() -> String {
        return "\x1b[33m".to_string();
    }
    pub fn bg_green() -> String {
        return "\x1b[32m".to_string();
    }
    pub fn bg_blue() -> String {
        return "\x1b[34m".to_string();
    }
    pub fn bg_white() -> String {
        return "\x1b[37m".to_string();
    }
    pub fn bg_cyan() -> String {
        return "\x1b[36m".to_string();
    }
    pub fn bg_black() -> String {
        return "\x1b[30m".to_string();
    }
    pub fn bg_magenta() -> String {
        return "\x1b[35m".to_string();
    }
    pub fn bg_reset() -> String {
        return "\x1b[0m".to_string();
    }
}