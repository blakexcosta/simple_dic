# Dictionary Implementation and Usage Guide
## Overview

This repository provides a simple dictionary implementation in Rust with various functions to manipulate 
and interact with the dictionary data structure. 
The dictionary allows you to store key-value pairs
where keys are of type String and values can be of any type T.

## Functions and Usage
###Include in project
```rs
use simple_dic::Dictionary;
```
### new() -> Self
This function creates a new instance of the Dictionary struct.
```rs
let mut my_dict = Dictionary::<T>::new();
```
### push(&mut self, key: String, value: T) -> Result<(), String>
Use this function to add a new key-value pair to the dictionary. 
If the key already exists, it returns an error message.
```rs
let key = "name".to_string();
let value = "John".to_string();

match my_dict.push(key, value) {
    Ok(()) => println!("Key-value pair added successfully."),
    Err(err) => println!("Error: {}", err),
}
```
### pop(&mut self)
This function removes the newest (last-added) key-value pair from the dictionary.
```rs
my_dict.pop();
```
### search(&self, key: String) -> bool
Use this function to check if a key exists in the dictionary.
```rs
if my_dict.search("name".to_string()) {
    println!("Key found!");
} else {
    println!("Key not found.");
}

```
### len(&self) -> usize
This function returns the number of key-value pairs in the dictionary.
```rs
let num_entries = my_dict.len();
println!("Number of entries in the dictionary: {}", num_entries);

```
### drop(&mut self, key: String) -> bool
This function deletes a key-value pair based on the provided key. It returns true if the key-value pair was found and deleted, otherwise false.
```rs
if my_dict.drop("name".to_string()) {
    println!("Key-value pair deleted successfully.");
} else {
    println!("Key-value pair not found.");
}

```
### contains(&self, value: &T) -> bool
This function checks if a given value exists in the dictionary's values.
```rs
if my_dict.contains(&"John".to_string()) {
    println!("Value found in the dictionary.");
} else {
    println!("Value not found.");
}

```
### raw_search(&self, key: String) -> Result<usize, String>
```
match my_dict.raw_search("name".to_string()) {
    Ok(index) => println!("Key found at index: {}", index),
    Err(err) => println!("Error: {}", err),
}

```
### overwrite(&mut self, key: String, newvalue: T) -> Result<(), String>
This function overwrites the value associated with a key. If the key is found, the value is updated. If the key is not found, an error is returned.
```rs
match my_dict.overwrite("name".to_string(), "Jane".to_string()) {
    Ok(()) => println!("Value updated successfully."),
    Err(err) => println!("Error: {}", err),
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
