fn main(){
    let a = [1,2,3,4,5];
    println!("a = {:?}", a);
    println!("first = {} second = {}", a[0], a[1]);
    let str = ["one","two", "three", "four", "five"];
    println!("str = {:?}", str);
    let statement : [i32; 5] = [1, 2, 3, 4, 5]; 
    println!("statement = {:?}", statement);
    let repeat = [1;5];
    println!("repeat = {:?}", repeat);

    let arr : [i32; 5] = [1,2,3,4,5];
    let slice : &[i32] = &arr[0..2];
    println!("slice = {:?}", slice);

    let mut mut_var = [1,1,1];

    mut_var[0] = 3;
    println!("mut = {:?}", mut_var)

}