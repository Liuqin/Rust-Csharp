pub mod sharp_helper;
use sharp_helper::int as int;
fn main() {
    println!("Hello, world!");
    let mut x:int=max(3,5);
     println!("{}",x);
    x=12;
    println!("{}",x);
}

fn max(a:int,b:int)->int{
    if a>b
    {
    return a;

    }
    else {
        return b;
    }
}
