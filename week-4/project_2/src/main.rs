// Rust program to determine the annual incentitive of employesss

use std::io;

fn main(){
    print!("\nEmployee Database Management System");

    let mut input1 = String::new();
    let mut input1 = String::new();

    // Get exeperience input
    print!("\nAre you experienced? ");
    io::stdin().read_line(&mut input1).expect("Not a string");
    let experience = input1.trim().to_lowercase();


// Get age input
    print!("\nEnter your age ");
    io::stdin().read_line(&mut input2).expect("This is not a string");
    let age:u32 = input2.trim().parse().expect("This is not a number")
    
    //Incentive
    let mut incentive:u32 = 0;

    if experience == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
            else if age >= 30 && age < 40 {
                incentive = 1_480_000;
                else if age < 28 {
                    incentive = 1_300_000;
                    else if age >= 28 && age < 30 {
                        // Not explicitly, but I would like to add a base
                           incentive = 1_400_00;
                           else {
                            incentive = 100_00;
                           }
                    }
                }
            }
        }
    println!("\nEmployee incentive Report");
    print!("\nYour incentive: {}",  incentive );
    }















}
