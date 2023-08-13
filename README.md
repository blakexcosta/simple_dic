#Dictionary Implementation and Usage Guide
##Overview

This repository provides a simple dictionary implementation in Rust with various functions to manipulate 
and interact with the dictionary data structure. 
The dictionary allows you to store key-value pairs
where keys are of type String and values can be of any type T.

##Functions and Usage

###new() -> Self
```
let mut my_dict = Dictionary::new();
```
###push(&mut self, key: String, value: T) -> Result<(), String>
```
let key = "name".to_string();
let value = "John".to_string();

match my_dict.push(key, value) {
    Ok(()) => println!("Key-value pair added successfully."),
    Err(err) => println!("Error: {}", err),
}
```
###pop(&mut self)
```
my_dict.pop();
```

