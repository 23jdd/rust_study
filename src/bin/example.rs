#[derive(Debug)]
struct Foo;

impl Foo {
    //    & mut 'a self  ->  &'a Self
    fn mutate_and_share(&mut self) -> &Self { &*self }
    fn share(&self) {}
}
/**
  fn main(){
       
  }
 */
fn main() {
    let  foo = Foo;
    let loan = &foo;
    foo.share();
    println!("{:?}", loan);
}