


pub trait Summary {
     fn summary(&self); 
}
struct A{
     
}
struct B{
     
}
impl Summary for A{
    fn summary(&self) {
        println!("A")
    }
}
impl Summary for B {
    fn summary(&self) {
        println!("B")
    }
}
/**
 ```rs
   println("hello wolrd")
 ```
 */
fn main(){
    let mut v:Vec<&dyn Summary>=Vec::new();
    v.push(&A{});
    v.push(&B{});
    for i in v{
        i.summary(); 
    }
}
 