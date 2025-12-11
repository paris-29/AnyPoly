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

fn main(){
    println!("hello User!");
    welcome_func();
    vars();
    var_simple_cal();
}
