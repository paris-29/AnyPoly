use std::io;

fn welcome_func() {
    println!("Welcome , do you know 2+2 = {} ", 2 + 2 ); //runing operations
}

fn vars() {
    let a :i32 = 56;
    let b :i32 = 44;
    println!("if a = {a} and b = {b} , therefore {a} + {b} = {}", {a} + {b});
}

fn var_simple_cal() {
    let mut v1 = String::new();
    let mut v2 = String::new();
    io::stdin().read_line(&mut v2).expect("err");
    io::stdin().read_line(&mut v1).expect("err reading");
    let v1 : i32 = v1.trim().parse().expect("err converting");
    let v2 : i32 = v2.trim().parse().expect("err");
    println!("Multiplying {v1} with {v2} , so answer is {} ", {v1} * {v2} );

}

fn calculator() {
println!("Starting Calculator");
println!("\nhere are the Following operators,
    1. '+' - to ADD values,
    2. '-' - to SUB values,
    3. '*' - to MULTIPLY values,
    4. '/' - to DIVIDE values,
    \n Write your EQN in the following format \n (num1) (opr) (num) " );

println!("\n Enter eqn --> ");

let mut inpt = String::new();

io::stdin().read_line(&mut inpt).expect("err reading line");

let inpt = inpt.trim();

let prts : Vec<&str> = inpt.split_whitespace().collect();

if prts.len() != 3 {
    println!("Error fill in the correct format");
    }

let n1: f64 = match prts[0].parse() {
    Ok(n) => n,
    Err(_) => todo!(),
};

let n2: f64 = match prts[2].parse() {
    Ok(n) => n,
    Err(_) => todo!(),
};

let result = 
if prts[1] == "+" {
     n1 + n2 }
else if prts[1] == "-" {
    n1 - n2 }
else if prts[1] == "*" {
  n1 * n2 }
else if prts[1] == "/" { 
    n1 / n2 }
else { 5.0 + 5.0 }; 
println!("so {} {} {} = {}", n1, prts[1], n2, result );
println!("Hello ");
}
fn main(){
    println!("hello User!");
    welcome_func();
    vars();
    var_simple_cal();
    println!("ran till here");
    calculator();
    println!("ran till here");
    //println!("so {} {} {} = {} ", n1, prts[1], n2, result );
}
