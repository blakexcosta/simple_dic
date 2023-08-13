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
     //Pushes a key/value pair to the end
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

        //search for a key and return an index
         pub fn raw_search(&self,key:String) -> Result<usize,String> {
            for i in 0..self.key.len(){
                 if self.key[i] == key {
                    return Ok(i)
               }
            }
            return Err(format!("Could not find key: {}",key));
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
        //checks if a value exists in the dictionary
        pub fn contains(&self,value:&T) ->bool{
            return self.value.contains(value);
        }
        
        //overwrite the value of the value of a key
        pub fn overwrite(&mut self,key:String,newvalue:T) -> Result<(),String>{
            match self.raw_search(key) {
               Ok(val) => {
                self.value[val] = newvalue;
                return Ok(());
                },
               Err(error) => {
                return Err(error)
                },       
            }     
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
    //testing searching and loops
   #[test]
    fn testcase1() {
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
    //testing containts function
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
    //testing the overwrite
    #[test]
    fn testcase4() {
        let mut obj = Dictionary::<String>::new();
        let mut f = false;
        for i in 0..100{
             let iclone = i.clone().to_string();
            obj.push(format!("a{}",iclone),format!("b{}",iclone));
         }
        obj.overwrite("a20".to_string(),"x20".to_string());
        f = obj.contains(&"x20".to_string());
        assert_eq!(f,true);
    }
}

