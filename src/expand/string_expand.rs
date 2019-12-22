pub trait IStringExpand {
   fn console(&self);
   fn index_of(&self,target:&String)->i32;
   fn sub_string(&self,index:i32,lenth:i32)->String;
   fn pand_left( &mut self,append:&str,lenth:i32)->String;
   fn pand_right(&mut self,append:&str,lenth:i32)->String;
   fn as_i32(&self)->i32;
   fn as_usize(&self)->usize;
   fn index_char_as_string(&self,index:i32)->String;
   fn as_mut(&self)-> &mut String;
}

pub trait IConvertExpand {
   fn as_i32(&self)->i32;
   fn as_usize(&self)->usize;
}





 impl IStringExpand for String {


     fn as_mut(&self)-> &mut String
     {
          let ptr: *mut String = &mut self.clone();  
          let y:&mut String= unsafe { &mut *ptr };
          return  y;
     }

     
    // 获取某个字符串中某个字符，转为String 类型
     fn index_char_as_string(&self,index:i32)->String{
      let index_usize:usize=index.to_string().as_usize();
      return self.chars().nth(index_usize).unwrap().to_string();
     }

   // 实现字符串快速打印
        fn console(&self){
            println!("{}",self.to_string());
        }


        // 实现字符串快速索引
        fn index_of(&self,target:&String)->i32 {
                let  n:i32=target.len() as i32;
                let  moveindex:i32=self.len() as i32-n;
                let mut x:i32=0;
                if Some(target)==None || !self.contains(target) || Some(self)==None || moveindex <0
                {
                    println!("error!");
                    return -1
            }
                while x <moveindex{
                    x=x+1;
                    let mut index=0;
                    let  realindex=index+x;
                    let mut chars_str=self.chars();
                    let mut chars_target=target.chars();
                        while index<= n 
                        {
                            if chars_str.nth(realindex as usize)==chars_target.nth(index as usize){
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
            if index >= 0 
            {
            let slice = &self[index as usize ..((lenth+index) as usize)];
            return slice.to_string();
            }
            else
            {
            let start_index=self.len() as i32 -lenth +index-1;
          //   println!("start_index:{}",start_index);
            if start_index >=0
            {
                let i_start=start_index as usize;
                let i_end=(start_index  +lenth) as usize;
                let slice =&self[i_start..i_end];
                return slice.to_string();
            }
            else {
                return self.to_string();
            }
                
            }
        }
        fn pand_right(&mut self,append:&str,lenth:i32)->String
        {
           let mut len=self.len() as i32;
           let append_len=append.len() as i32;
           let real_size =lenth as usize;
           if len <lenth
           {
            while len< lenth 
            {
                len=len+append_len;
                self.push_str(append);
                if len>lenth
                {
                    break;
                }
            }
           }
              return (&self[..real_size]).to_string()
        }
        fn pand_left(&mut self,append:&str,lenth:i32)->String
        {

           let mut len=self.len() as i32;
           let append_len=append.len() as i32;
           let real_size =lenth ;
           let mut append_str=String::from("");
           if len <lenth
           {
            while len< lenth 
            {
                len=len+append_len;
                append_str.push_str(append);
                if len>lenth
                {
                    break;
                }
            }
             append_str.push_str(&self.to_string());
             println!("{}",append_str);
             println!("{}",real_size);
             let y =append_str.sub_string(-1,real_size);
           //  y.console();
             return y;
           }
           else {
             return  self.sub_string(-1,real_size);
           }

        }

         fn as_i32(&self)->i32{
       let x=self.to_string();
       return x.parse::<i32>().unwrap() as i32;
   }
    fn as_usize(&self)->usize{
       let x=self.to_string();
       return x.parse::<i32>().unwrap() as usize;
   }

}

impl  IConvertExpand for usize {
    
   fn as_i32(&self)->i32{
       let x=self.to_string();
       return x.parse::<i32>().unwrap() as i32;
   }
    fn as_usize(&self)->usize{
       let x=self.to_string();
       return x.parse::<i32>().unwrap() as usize;
   }
}
