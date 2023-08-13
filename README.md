# Dictionary Implementation and Usage Guide
## Overview

This repository provides a simple dictionary implementation in Rust with various functions to manipulate 
and interact with the dictionary data structure. 
The dictionary allows you to store key-value pairs
where keys are of type String and values can be of any type T.

## Functions and Usage

### new() -> Self
```rs
let mut my_dict = Dictionary::new();
```
### push(&mut self, key: String, value: T) -> Result<(), String>
```rs
let key = "name".to_string();
let value = "John".to_string();

match my_dict.push(key, value) {
    Ok(()) => println!("Key-value pair added successfully."),
    Err(err) => println!("Error: {}", err),
}
```
### pop(&mut self)
```rs
my_dict.pop();
```
### search(&self, key: String) -> bool
```rs
if my_dict.search("name".to_string()) {
    println!("Key found!");
} else {
    println!("Key not found.");
}

```
### len(&self) -> usize
```rs
let num_entries = my_dict.len();
println!("Number of entries in the dictionary: {}", num_entries);

```
### drop(&mut self, key: String) -> bool
```rs
if my_dict.drop("name".to_string()) {
    println!("Key-value pair deleted successfully.");
} else {
    println!("Key-value pair not found.");
}

```
### contains(&self, value: &T) -> bool
```rs
if my_dict.contains(&"John".to_string()) {
    println!("Value found in the dictionary.");
} else {
    println!("Value not found.");
}

```
## Example
```rs
use dictionary::Dictionary;

fn main() {
    let mut my_dict = Dictionary::new();
    
    my_dict.push("name".to_string(), "John".to_string()).unwrap();
    my_dict.push("age".to_string(), 25).unwrap();

    println!("Dictionary contains 'name': {}", my_dict.search("name".to_string()));
    println!("Dictionary length: {}", my_dict.len());

    my_dict.pop();
    println!("Dictionary length after pop: {}", my_dict.len());

    my_dict.drop("age".to_string());
    println!("Dictionary length after drop: {}", my_dict.len());
}

```
