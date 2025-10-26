// Rust program to solve quadratic roots and determinants

use std::io;

fn main(){
    println!("\nFind the roots of the equation");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

println!("\nEnter the value of a:");
io::stdin().read_line(&mut input1).execpt("Not a valid string");
let a:f32 = input1.trim().parse().execpt("Please enter a integer");

println!("\nEnter the value of b:");
io::stdin().read_line(&mut input2).execpt("Not a valid string");
let a:f32 = input2.trim().parse().execpt("Please enter a integer");


println!("\nEnter the value of c:");
io::stdin().read_line(&mut input3).execpt("Not a valid string");
let a:f32 = input3.trim().parse().execpt("Please enter a integer");

//computing the discriminant
let discriminant = b.powf(2.0) - _(4.0*a*c);
println!("\nThe discriminant is: {}", discriminant);

if discriminant > 0.0 {
    let root = -b / (2.0 = a);
    print!("Your equation has one real root: x = {}", root);


} else{
    print!("Your equationhas no real roots");
}
}