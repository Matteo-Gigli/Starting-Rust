/*
Starting Rust, Declaring and Print a Variables
Little rules to follow:
    1 Never use CamelCase because is not allowed
    2 Use snake_case
    3 Strings are always declared with "" and never with ''
*/

// This const and function bigger will need it later

const LIMIT:i32=12;

fn bigger(n:i32) -> bool{

    n > LIMIT
}

//  Always start with main function

fn main() {

    // Starting Rust with Declaration and Print
    let my_number = 15;
    println!("My number is {}", my_number);
    println!("");

    let my_string = "My name is John";
    println!("{}", my_string);
    println!("{} , and i have {} years old", my_string, my_number);
    println!("");

    // Mutables Variables check below
    let mut counting = 12;
    println!("This is the counting at this moment {}", counting);
    counting = 13;
    println!("Now Counting is this {}", counting);
    println!("");

    /*
    Change Variables and delete mut, Will not Working because default variables are not mutable, in fact we
    cannot change the value associate in this case.
    Different from upper example.

    let counting = 12;
    println!("This is the counting at this moment {}", counting);
    counting = 13;
    println!("Now Counting is this {}", counting);


    This code is different and will work, because we are re-declaring our variables.

    let counting = 12;
    println!("This is the counting at this moment {}", counting);
    let counting = 13;
    println!("Now Counting is this {}", counting);

    */

    // Boolean Value and Char value

    let my_truth: bool = true;
    let letter: char = 'a';
    println!("Think it's {}", my_truth);
    println!("This is the alphabets first letter {}", letter);
    println!("");


    /*
    Now will need of const and function 'bigger' setted on top.
    We create a little compare about the limit and a number we are going to set now, if number is bigger
    give back a message "BIGGER" else a message "Smaller"
    */

    let comparing_number = 10;
    println!("Our Limit is {}", LIMIT);
    println!("You are giving me {}", comparing_number);
    print!("Your number is {} {}", comparing_number,
           if bigger(comparing_number){"BIGGER"}else{"SMALLER"});
    println!("");

    // Acquiring just the first 2 decimals of a float number

    let floating_number = 12.3456789;
    println!("{:.2}", floating_number);

    /*
     Hex, Binary, Octal numbers
     We can even pass arguments like this.
     In this case we have in output Hex about first 10,
     Bin about second 10,
     Octal about third 10
    */
    println!("Hex:{:x}, Bin:{:b}, Octal:{:o}", 10, 10, 10);
    println!("");

    // Math Operations as + - * / %

    println!("5 + 4 is {}", 5+4);
    println!("5 - 4 is {}", 5-4);
    println!("3 * 3 is {}", 3*3);
    println!("9 / 3 is {}", 9/3);
    println!("9 % 4 is {}", 9%4);
    println!("");


    //absolute of a number
    let negative = -4i32;
    println!("Absolute number of {} is {}", negative, negative.abs());
    println!("");

    // Can have a pow of a Number
    println!("Pow 2 for {} number is {}", negative, negative.pow(2));
    println!("");

    //Rounded Number to the closer, integer Number
    println!("Rounded for 1.23456789 is {}", 1.23456789_f32.round());
    println!("");


    //Rounded number for the closer, integer, bigger Number
    println!("Rounded Upper Number for 1.23456789 is {}", 1.23456789_f32.ceil());
    println!("");


    //Start Working with Strings

    let new_string = "This is my new string";

    // check the string lenght via .len()
    println!("String Lenght = {}", new_string.len());

    //spilt a string first method
    println!("This \n is\n new string");

    //split a string second method
    let  my_new_string = "Programming is very Cool";
    let (first_part, second_part) = my_new_string.split_at(7);
    println!("First part is '{0}' and second part is '{1}'", first_part, second_part );
    println!("");


    //Starting work with Array

    let my_array = [2, 5, 7, 9, 11];
    println!("This is my Array {:?}", my_array);

    //Would like to know the array lenght
    println!("Array Lenght is {}", my_array.len());

    //Would take only the 7 and 9 from my_array
    println!("This are numbers in the middle of my array{:?}", &my_array[2..4]);
    println!("");


    //Working with vector
    let mut my_vector = vec! [1, 2, 3, 5, 7, 9, 11];
    println!("My vector is {:?}", my_vector);
    println!("This is the third element of my vector {}", my_vector[2]);

    //Iterate and Print one by one

    for i in &my_vector{
        println!(" => {}", i);
    }
    println!("");

    //Add an element for a vector is like

    let mut my_new_vector = vec! [1, 5, 8, 0, 4, 9];
    println!(" My vector is {:?}", my_new_vector);
    my_new_vector.push(7);
    println!("We added an item, now vector is {:?}", my_new_vector);

    //Delete last element of vector

    my_new_vector.pop();
    println!("After Pop vector is {:?}", my_new_vector);
    println!("");

    //Tuples

    let my_tuple = ("Rust", 2022);
    let my_second_tuple: (&str, i16) = ("Python", 2020);
    println!("I'm learning {}", my_tuple.0);
    println!("I learned Python in {}", my_second_tuple.1);
    println!("");

    //if, else if and else

    let age = 18;
    let response: bool=false;

    if age < 18{
        println!("Your age is {} and you cannot drive", age);
    }else if(age >= 18) && (age <= 19){
        println!("Did you already have the driving license?");
        if response == true{
            println!("Ok perfect we can go")}
            else{
                println!("Maybe should be better if you take that first");
            }
    }else if(age < 18 && response == true){
        println!("Your driving license is false");
    }else{
        println!("We did all the check so now we can go");
    }

    //We can declare an if else condition in one line

    let vote = if (age >= 18 && response == false){"You got vote and not car!"}
    else if(age >= 18 && response == true){"Car and Vote... Huge "}
    else if(age < 18 && response == true){"Your driving license is false"}else{"No car and No vote"};

    println!("{}", vote);


    // loop, while, for

    let mut x = 1;

    loop{
        if(x % 2 == 0){
            println!("Number {} is odd", x);
            x+=1;
            continue;
        }
        if (x > 10){
            break;
        }
        x+=1;
    }

    // While

    let mut y = 1;

    while y < 10{
        println!("Number {}", y);
        println!("");
        y+=1;
    }

    // For
    let mut z = 1;
    for z in 1..10{
        println!("For => {}", z);
        println!("");
    }

}
