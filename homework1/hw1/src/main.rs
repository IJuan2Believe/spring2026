// assignment 1 :
/* Declare a constant for the freezing point of water in Fahrenheit (32°F).
Implement two functions:
fahrenheit_to_celsius(f: f64) -> f64: Converts Fahrenheit to Celsius
celsius_to_fahrenheit(c: f64) -> f64: Converts Celsius to Fahrenheit
In the main function:
Declare a mutable variable with a temperature in Fahrenheit
Convert it to Celsius using your function and print the result
Use a loop to convert and print the next 5 integer temperatures 
(e.g., if you start with 32°F, print conversions for 33°F, 34°F, 35°F, 36°F, and 37°F) */

//creating the functions
const freezing_point_f: f64 = 32.0;
fn fahrenheit_to_celsius(f: f64) -> f64 {
 ( f - freezing_point_f) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64{
( c * 9.0 / 5.0 ) + freezing_point_f 
}



/*Assignment 2: Number Analyzer
Create a Rust program that analyzes a series of numbers. The program should:

Create an array of 10 integer numbers of your choice.
Implement a function is_even(n: i32) -> bool that returns true if a number is even, false otherwise.
Use a for loop to iterate through the array and for each number:
Print whether it's even or odd using your is_even function
If the number is divisible by 3, print "Fizz" instead
If the number is divisible by 5, print "Buzz" instead
If it's divisible by both 3 and 5, print "FizzBuzz"
Use a while loop to find and print the sum of all numbers in the array.
Use a loop to find and print the largest number in the array.

*/ 

 

fn is_even(n: i32)-> bool{
n % 2 == 0 
}


fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret {
        0
    } else if guess > secret {
        1 
    } else {
        -1        
    }
}


fn main() {
    /* Problem 1 ###############################################################################
    let mut temp_fahrenheit: f64 = freezing_point_f;  
    let temperature_celsius = fahrenheit_to_celsius(temp_fahrenheit);
    println!("{temp_fahrenheit}°F is equal to {}°C", temperature_celsius);

    // creating a loop to print the next 5 values 
    for i in 1..=5{
        let current_f = temp_fahrenheit + i as f64;
        let current_c = fahrenheit_to_celsius(current_f);
        println!("{current_f}°F is equal to {:.2}°C", current_c);
    }
    #################################################################################################*/ 
/* 
    let number_array: [i32; 10] = [3,4,5,6,7,8,9,10,11,12];
    //fizzbuzz for loop
    for num in number_array.iter() { 
        if *num % 3 == 0  && *num % 5 == 0 {
            println!("number {} = Fizzbuzz", num);
        } else if *num % 3 == 0 {
            println!("number {} = Fizz", num);
        } else if *num % 5 == 0 {
            println!("number {} = Buzz", num);
        } else if is_even(*num){
            println!("number {} is even", num);
        } else {
            println!("number {} is odd",  num );
        }
    }
    // while loop to get sum value:
    let mut sum = 0 ;
    let mut counter =  0 ; 

    while counter < 10 {
        sum += number_array[counter];
        counter += 1 ; 

    }
    println!("the total sum was {}" , sum);

    // making a loop that finds the biggest number
    let mut biggest_number = number_array[0];
    for &num in number_array.iter(){
        if num > biggest_number { 
            biggest_number = num 
        }
    }
    println!("the biggest number was: {} ", biggest_number);
 *//* 
 Assignment 3: Guessing Game
Create a simple number guessing game in Rust. The program should:

 
In the main function:
Use a loop to repeatedly:
Set a mutable guess variable to a number of your choice (simulating user input)
Call the check_guess function
Use an if-else expression to print whether the guess was correct, too high, or too low
If the guess was correct, break the loop
After the loop ends, print how many guesses it took (you'll need to track this in a variable)
*/

let secret: i32 = 25;
let mut attempts: i32 = 0 ;
let mut guess: i32 = 0 ;

   while guess != secret {
        attempts += 1;

        let result = check_guess(guess, secret);

        if result == 1 {
            println!("Guess {} is too high", guess);
        } else {
            println!("Guess {} is too low", guess);
        }

        guess += 1;
    }

    // Count the final correct guess
    attempts += 1;
    println!("Correct! It took {} guesses.", attempts);
}



