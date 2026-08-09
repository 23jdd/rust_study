fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    let it = a.into_iter().filter(|x: &i32| *x > 2);
    for i in it {
        println!("{}", i)
    }
}
