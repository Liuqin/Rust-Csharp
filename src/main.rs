pub mod sharp_helper;
use sharp_helper::int as int;
use sharp_helper::string as string;
fn main() {
    println!("Hello, world!");
    let mut x:int=max(3,5);
     println!("{}",x);
    x=12;
    println!("{}",x);
    x=index_of(&string::from("aaaaaa"),&string::from("sd"));
     println!("{}",x);
    let x1=String::from("123456");
    let mut x2=sub_string(&x1,-1,2);
      println!("{}",x2);
      let  y1=2;
      x2=sub_string(&x1,-1, y1);
       println!("{}", y1);
       println!("{}",x2);

    
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

fn sub_string(str:&String,index:i32,mut sub_len:i32)->String
{
    if Some(str) == None || str.len() < ((index.signum() +sub_len) as usize)
    {
        return String::from("");
    }
    if index >= 0 {
    let slice = &str[index as usize ..((sub_len+index) as usize)];
    return slice.to_string();
    }
    else
    {
     let start_index=str.len() as i32 -sub_len +index-1;
      println!("start_index:{}",start_index);
     if start_index >0
     {
         let i_start=start_index as usize;
         let i_end=(start_index  +sub_len) as usize;
         let slice =&str[i_start..i_end];
           sub_len=1;
           println!("sub_len change:{}",sub_len);
         return slice.to_string();
     }
     else {
         return String::from("");
     }
        
    }

}
