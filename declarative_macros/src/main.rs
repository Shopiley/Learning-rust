mod lib;
fn main() {
    let vec1 = veco![1, 2, 3, 4];
    let _vec2 = veco!["a", "b", "c"];

    for x in vec1{
        print!("{}", x);
    } 
}
