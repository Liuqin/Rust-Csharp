
use crate::expand::string_expand::IStringExpand;
mod sharp_helper;
mod expand;
// use sharp_helper::int as int;
// use sharp_helper::string as string;

fn main() {

    String::from("1232313").console();
    String::from("123456").index_of(&String::from("234")).to_string().console();
    String::from("1234567").sub_string(1,3).console();

    
}



