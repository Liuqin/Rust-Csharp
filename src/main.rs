
use crate::expand::string_expand::IStringExpand;
use crate::expand::string_expand::IConvertExpand;
mod sharp_helper;
mod expand;
// use sharp_helper::int as int;
// use sharp_helper::string as string;

fn main() {

    String::from("1232313").console();
    String::from("123456").index_of(&String::from("234")).to_string().console();
    String::from("1234567").sub_string(1,3).console();
    let m=String::from("abcdefg");
    let y=m.sub_string(-1,3);
    y.console();
    let x=y.index_of(&String::from("d")).to_string();
    x.console();
    let mut r= String::from("123456");
    r.pand_left(&String::from("$"),r.len().as_i32()+1).console();
    String::from("123").as_i32().to_string().console();
    let mut str_test=String::from("ABCDEFGHIJK");
    //打印每个字符
    for i in 0..str_test.len(){
       str_test.index_char_as_string(i.as_i32()).console();
       i.to_string().console();
       str_test.console();
    }

}



