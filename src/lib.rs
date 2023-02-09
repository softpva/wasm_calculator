use std::{collections::VecDeque, io::{self, Write}};
use wasm_bindgen::prelude::*;
mod utils;

use wasm_bindgen::prelude::*;

fn eval_rpn(rpn: &str) -> f64 {
    let mut stack = VecDeque::new();

    for token in rpn.split_whitespace() {
        match token {
            "+" => {
                let b: f64 = stack.pop_back().unwrap();
                let a = stack.pop_back().unwrap();
                stack.push_back(a + b);
            }
            "-" => {
                let b = stack.pop_back().unwrap();
                let a = stack.pop_back().unwrap();
                stack.push_back(a - b);
            }
            "*" => {
                let b = stack.pop_back().unwrap();
                let a = stack.pop_back().unwrap();
                stack.push_back(a * b);
            }
            "/" => {
                let b = stack.pop_back().unwrap();
                let a = stack.pop_back().unwrap();
                stack.push_back(a / b);
            }
            "**" => {
                let b = stack.pop_back().unwrap();
                let a = stack.pop_back().unwrap();
                stack.push_back(a.powf(b));
            }
            _ => {
                let num = token.parse().unwrap();
                stack.push_back(num);
            }
        }
    }

    stack.pop_back().unwrap()
}

fn infix_to_rpn(infix: &str) -> String {
    let mut stack = Vec::new();
    let mut output = String::new();

    for token in infix.split_whitespace() {
        match token {
            "(" => stack.push(token),
            ")" => {
                while let Some(op) = stack.pop() {
                    if op == "(" {
                        break;
                    }
                    output.push_str(&op);
                    output.push(' ');
                }
            }
            "+" | "-" => {
                while let Some(op) = stack.last() {
                    if op == &"(" {
                        break;
                    }
                    output.push_str(stack.pop().unwrap());
                    output.push(' ');
                }
                stack.push(token);
            }
            "*" | "/" => {
                while let Some(op) = stack.last() {
                    if op == &"(" || op == &"+" || op == &"-" {
                        break;
                    }
                    output.push_str(stack.pop().unwrap());
                    output.push(' ');
                }
                stack.push(token);
            }
            "**" => {
                while let Some(op) = stack.last() {
                    if op == &"(" || op == &"+" || op == &"-" || op == &"*" || op == &"/" {
                        break;
                    }
                    output.push_str(stack.pop().unwrap());
                    output.push(' ');
                }
                stack.push(token);
            }
            _ => {
                output.push_str(token);
                output.push(' ');
            }
        }
    }

    while let Some(op) = stack.pop() {
        output.push_str(&op);
        output.push(' ');
    }

    output
}

#[wasm_bindgen]
pub fn solve_infix(infix: &str) -> f64 {
    eval_rpn(&infix_to_rpn(infix))
}

// fn main() {
//     // for test
//     let mut txt = String::new();
//     println!("Write an infix expression separated by spaces
//     Example: \"( 1 + 1 ) ** ( 2 + 1 ) + 10 / 5\" \n"
//     );
//     print!("->");
//     io::stdout().flush().unwrap();
//     std::io::stdin()
//         .read_line(&mut txt)
//         .expect("Failed to read line");
//     println!("{}", infix_to_rpn(&txt));
//     println!("{}", solve_infix(&txt).to_string());
// }
