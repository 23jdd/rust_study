use std::collections::HashMap;

type Store=Vec<String>;
/// a example todo
fn main(){
    let mut  map:HashMap<String,String> = HashMap::new();
    map.insert("hello".to_string(), "wolrd".to_string());
    match map.get(&"hello".to_string()) {
        Some(_) => {},
        None => {},
    }
    let mut v=vec![1,2];
    v[0]=1;    
}