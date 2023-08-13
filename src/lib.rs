 struct Dictionary<T> {
        key: Vec<String>,
        value: Vec<T>,
    }

    impl<T: std::cmp::PartialEq> Dictionary<T> {
    //Function to create an instance of a dictionary
        pub fn new() -> Self {
            Dictionary {
                key: Vec::new(),
                value: Vec::new(),
            }
        }
        pub fn push(&mut self,key:String,value:T) -> Result<(),String> {
                if self.key.contains(&key){
                return Err(format!("Key: '{}' does not exist.",key))    
            }else{
                self.key.push(key);
                self.value.push(value);
                Ok(())
            } 
        }
        //removes newest element from Dic
        pub fn pop(&mut self){
            self.key.pop();
            self.value.pop();
        }
        
        //search for a key
        pub fn search(&mut self, key:String) -> bool {
            if self.key.contains(&key) {
                return true;
            }else{
                return false;
            }
        } 
        //returns a usize with the length of the dictionary
        pub fn len(&self) -> usize {
            return self.key.len();
        }
        
        //deletes the key and value with a key passed into the function
        pub fn drop(&mut self,key:String) -> bool{
               if let Some(index) = self.key.iter().position(|x| x == &key){
                    self.key.remove(index);
                    self.value.remove(index);
                    return true;
                }else{
                    return false;
                }
        }

        pub fn contains(&self,value:&T) ->bool{
            return self.value.contains(value);
        }
    }
    

#[cfg(test)]
mod tests {
    use crate::Dictionary;
     #[test]
     fn testcase0() {
            //testing if length works
    let mut obj = Dictionary::<&str>::new();
    let mut  f = 0;

    for i in 0..100{
        let iclone = i.clone().to_string();
        obj.push(format!("a{}",iclone),"Value");
        f = f + 1
    }
    assert_eq!(f,obj.len());
}
   #[test]
    fn testcase1() {
         //searching a key from a large dic
        let mut obj = Dictionary::<&str>::new();
        let mut  f = false;

         for i in 0..100{
             let iclone = i.clone().to_string();
            obj.push(format!("a{}",iclone),"Value");
         }
         f = obj.search("a51".to_string());
         assert_eq!(f,true);
    }
     //testing drop function
     #[test]
    fn testcase2() {          
    let mut obj = Dictionary::<&str>::new();
    let mut  f = 0;
    for i in 0..100{
        let iclone = i.clone().to_string();
        obj.push(format!("a{}",iclone),"Value");
        f = f + 1
    }
    obj.drop("a51".to_string());
    assert_eq!(f-1,obj.len());
    }
     #[test]
     fn testcase3() {
        let mut obj = Dictionary::<String>::new();
        let mut  f = false;

         for i in 0..100{
             let iclone = i.clone().to_string();
            obj.push(format!("a{}",iclone),format!("b{}",iclone));
         }
         f = obj.contains(&"b20".to_string());
         assert_eq!(f,true);
       
     }
}
