use rand::Rng;
use std::fs::File;
use std::io::{self,BufRead};


fn main() {


    // step 1: Display Game Name
println!("Welcome to Jmblue Word!");

    let mut ans = String::new();
    show_jumbled_word();


    //    let mut confirm: Vec<String> = Vec::new();
    // let check:Vec<String> = ans.chars().map(|c| c.to_string()).collect();
    //let mut i =0;
    //for check in check {
    //  i+=1;
    //}
    get_jumbled_word(ans);
    //word_check();


    // Ok(())
}

fn show_jumbled_word() {

    let mut randomizer = rand::thread_rng();

    println!("Guess the word! : ");

    for n in 0..10{
        let random_char = randomizer.gen_range(98.. 122);
        print!(" {} \t",char::from(random_char));
    }

}

fn get_jumbled_word(my_word: String){

    let mut ans= my_word;
    println!("\nEnter the word : ");
    io::stdin().read_line(& mut ans).expect("Invalid Input");
    let mut ans = ans.trim();
    let mut check:Vec<String> = ans.chars().map(|c|c.to_string()).collect();
    println!("\n{:?} ", check);

}

/*fn word_check(){
    let mut ind = false;

    let file = File::open("words.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        if ans == line {
            ind = true;
            println!("\nThe Answer is Correct !!!\nCongratulations!!!\nYou Win!!!  : {}",line);
        }
    }
    if ind == false {
        println!("There is no such word!!!\nYou Failed :)!!! ")
    }
}*/



