struct foo<'a>{
      name:&'a str 
}
fn debug<'a>(a:&'a str,b:&'a str){ 
      println!("{}:{}",a,b) 
}
fn display<'a>(f:foo<'a>){
    println!("{}",f.name)
}
fn main(){
    let a:&'static str="hello";
    {
        let b=String::from("world");
        debug(a, &b);
    }
    
}