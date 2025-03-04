mod compiler;
mod parser;
mod interpreter;
mod ast;
mod types;
mod features;
mod modules;
mod errors;
mod library;

use std::fs;
use log::{info, error};
use parser::lexer::Lexer;
use parser::parser::Parser;
use interpreter::evaluate::Interpreter;

#[tokio::main]
async fn main() {
    
}