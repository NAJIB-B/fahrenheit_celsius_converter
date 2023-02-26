use std::io;
fn main() {
    println!("Welcome to temperature converter");
    println!("press 1 to convert celsius to fahrenheit");
    println!("press 2 to convert fahrenheit to celcius");
   
    'main_loop: loop{
              
        let starting_message =String::from("starting..."); 
        // take user input
       let user_input = user_input_converter(starting_message);
        if user_input == 1.0 {
            loop{
                let instruction: String = String::from("input celcius value, input a valid number");
                let  user_input = user_input_converter(instruction);
               
             c_to_f(user_input);
             break 'main_loop;
            };

        }else if user_input == 2.0 {
            loop{
                let instruction: String = String::from("input fahrenheit value, input a valid number");
                let  user_input = user_input_converter(instruction);
    
             f_to_c(user_input);
             break 'main_loop;
            };

        }else{
            println!("please input 1 or 2");
            continue;
        };
    };
   
    
        
}

fn c_to_f(value:f64){
 let answer = (value * 9.0/5.0) + 32.0;
println!("{value} degree is = {answer} fahrenheit");
}
fn f_to_c(value:f64){

let answer = (value - 32.0) * 5.0/9.0;
println!("{value} fahrenheit is = {answer} degrees");
}

fn user_input_converter(instruction:String)->f64{
    let  result;
     loop {
         println!("{instruction}");
        let mut user_input = String::new();
    
    io::stdin()
              .read_line(&mut user_input)
              .expect("failed to read user input");
     
    let  user_input:f64 = match user_input.trim().parse() {
        Ok(num)=>num,
        Err(_)=> continue,
    };
    result = user_input;
    break;
    };

  return result;
   
}