pub mod sharp_helper;
use sharp_helper::int as int;
use sharp_helper::string as string;
fn main() {
    println!("Hello, world!");
    let mut x:int=max(3,5);
     println!("{}",x);
    x=12;
    println!("{}",x);
    x=index_of(&string::from("121l2jjdljsjflsdjfsd"),&string::from("21"));
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

fn index_of(strs:&String,target:&String)->int
{
 let  n:i32=target.len() as i32;
 let  moveindex:i32=strs.len() as i32-n ;
 let mut x:i32=0;
 if Some(target)==None || !strs.contains(target) || Some(strs)==None || moveindex <0
 {
     return -1
 }

 while x <moveindex
 {
  x=x+1;
  let mut index=0;
  let  realindex=index+x;
  let mut chars_str=strs.chars();
  let mut chars_target=target.chars();
  while index< n 
  {
      if chars_str.nth(realindex as usize)==chars_target.nth(index as usize)
      {
          index=index+1;
          if index== n-1
          {
            return x as int;
          }
      }
      else {
         break;
      }
  }
 }
 return -1;
}
