use std: :10;

fn main() {
printin/("\nEmployee Database Management System");

Let mut input1 = String :: new();
Let mut input2 = String :: new();

// Get experience input
println!("\nAre you experienced? ");
io :: stdin().read_line(&mut input1).expect("Not a string");
let experience = input1.trim().to_lowercase();

// Get age input
printLn!("\nEnter your age:");
io :: stdin().read_line(&mut input2).expect("This is not a string");
Let age:u32 = input2.trim().parse().expect("This is not a number");

//Incentive
Let mut incentive:u32 = 0;

if experience == "yes" {
if age >= 40 {
incentive = 1_560_000;
} else if age >= 30 && age < 40 {
incentive = 1_480_000;
} else if age < 28 {
| incentive = 1_300_000;
} else if age >= 28 && age < 30 {
// Not explicitly stated, but I would like to add a base
|incentive = 1_400_000;

} else {
incentive = 100_000;

printin!("\nEmployee Incentive Report");
printin/("\nYour incentive: (}", incentive);