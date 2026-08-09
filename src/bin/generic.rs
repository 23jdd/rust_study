struct Node<T> {
    val: T,
}
impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node { val }
    }
}
trait Next{
    type Item;
    fn next(&self)->Option<Self::Item>;
}

fn foreach(i:impl Next<Item=i32>){
    while let Some(v)=i.next(){
          println!("{}",v)  
    }
}
fn main(){
    
}
 