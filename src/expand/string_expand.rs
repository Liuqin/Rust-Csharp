pub trait IStringExpand {
   fn console(&self);
   fn index_of(&self,target:&String)->i32;
   fn sub_string(&self,index:i32,lenth:i32)->String;
}


 impl IStringExpand for String {

   // 实现字符串快速打印
   fn console(&self){
       println!("{}",self.to_string());
   }


   // 实现字符串快速索引
   fn index_of(&self,target:&String)->i32
{
 let  n:i32=target.len() as i32;
 let  moveindex:i32=self.len() as i32-n;

 let mut x:i32=0;
 if Some(target)==None || !self.contains(target) || Some(self)==None || moveindex <0
 {
     println!("error!");
     return -1
 }


//  println!("movecount:{},source:{},target:{}",moveindex,&self,&target);
 while x <moveindex
 {
  x=x+1;
  let mut index=0;
  let  realindex=index+x;
  let mut chars_str=self.chars();
  let mut chars_target=target.chars();
  while index<= n 
  {
      if chars_str.nth(realindex as usize)==chars_target.nth(index as usize)
      {
        //   println!("index:{},n:{}",index,n);
           index=index+1;
          if index >= n-1
          {
            return x as i32;
          }
         
      }
      else {
         break;
      }
  }
 }
 return -1;
}

// 实现字符串截断
fn sub_string(&self,index:i32,lenth:i32)->String
{
    if Some(self) == None || self.len() < ((index.signum() +lenth) as usize)
    {
        return String::from("");
    }
    if index >= 0 {
    let slice = &self[index as usize ..((lenth+index) as usize)];
    return slice.to_string();
    }
    else
    {
     let start_index=self.len() as i32 -lenth +index-1;
     // println!("start_index:{}",start_index);
     if start_index >0
     {
         let i_start=start_index as usize;
         let i_end=(start_index  +lenth) as usize;
         let slice =&self[i_start..i_end];
         return slice.to_string();
     }
     else {
         return String::from("");
     }
        
    }

   
}








}
