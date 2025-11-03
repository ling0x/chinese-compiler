// main.rs
use chinese_compiler::{ChineseLangParser, Rule, chinese_number::chinese_to_number};
use inkwell::context::Context;
use pest::Parser;
use std::fs;
use std::process::Command;

mod codegen;
use codegen::Compiler;

fn main() {
    let source = fs::read_to_string("programs/poem.zh").expect("无法读取文件");
    println!("Source file content:\n{}", source);

    // Parse the Chinese code
    let pairs = ChineseLangParser::parse(Rule::PROGRAM, &source).expect("解析错误");
    println!("Parsed successfully!");

    // Create LLVM context and compiler
    let context = Context::create();
    let mut compiler = Compiler::new(&context, "chinese_program");

    compiler.declare_printf();
    compiler.create_main_function();

    // Compile each statement
    for pair in pairs {
        if pair.as_rule() == Rule::PROGRAM {
            for statement_pair in pair.into_inner() {
                compile_statement(statement_pair, &mut compiler);
            }
        }
    }

    compiler.finish_main();

    // Output files
    compiler.write_llvm_ir("output.ll");
    println!("✓ Generated LLVM IR: output.ll");

    compiler.write_object_file("output.o");
    println!("✓ Generated object file: output.o");

    // Link to executable
    let status = Command::new("clang")
        .args(["output.o", "-o", "program"])
        .status()
        .expect("Failed to link");

    if status.success() {
        println!("✓ Generated executable: ./program");
        println!("\nRunning compiled program:");
        Command::new("./program").status().unwrap();
    }
}

fn compile_statement(pair: pest::iterators::Pair<Rule>, compiler: &mut Compiler) {
    println!("Compiling statement: {:?}", pair.as_rule()); //
    match pair.as_rule() {
        Rule::STATEMENT => {
            for inner in pair.into_inner() {
                compile_statement(inner, compiler);
            }
        }
        Rule::VAR_DECL => {
            let mut inner = pair.into_inner();
            inner.next(); // Skip LET_KW
            let var_name = inner.next().unwrap().as_str();
            let value_pair = inner.next().unwrap();

            // Evaluate the value as a string (for now)
            let value_str = evaluate_to_string(value_pair);
            println!("Variable: {} = {}", var_name, value_str);
            compiler.compile_string_var(var_name, &value_str);
        }
        Rule::PRINT_STMT => {
            let mut inner = pair.into_inner();
            inner.next(); // Skip PRINT_KW
            let value_pair = inner.next().unwrap();
            let var_name = extract_var_name(value_pair);
            println!("Printing variable: {}", var_name);
            compiler.compile_print_string(&var_name);
        }
        _ => {}
    }
}

fn extract_var_name(pair: pest::iterators::Pair<Rule>) -> String {
    match pair.as_rule() {
        Rule::VALUE | Rule::EXPRESSION | Rule::TERM => {
            let inner = pair.into_inner().next().unwrap();
            extract_var_name(inner)
        }
        Rule::VAR_NAME => pair.as_str().to_string(),
        _ => panic!("Expected variable name"),
    }
}

fn evaluate_to_string(pair: pest::iterators::Pair<Rule>) -> String {
    match pair.as_rule() {
        Rule::VALUE | Rule::EXPRESSION => {
            let inner = pair.into_inner();
            let mut result = String::new();
            for term in inner {
                result.push_str(&evaluate_term(term));
            }
            result
        }
        _ => panic!("Unexpected rule: {:?}", pair.as_rule()),
    }
}

fn evaluate_term(pair: pest::iterators::Pair<Rule>) -> String {
    match pair.as_rule() {
        Rule::EXPRESSION => evaluate_to_string(pair),
        Rule::TERM => {
            let inner = pair.into_inner().next().unwrap();
            evaluate_term(inner)
        }
        Rule::STRING => {
            let s = pair.as_str();
            s[1..s.len() - 1].to_string()
        }
        Rule::NUMBER => {
            let num_str = pair.as_str();
            if num_str.chars().next().unwrap() as u32 > 127 {
                chinese_to_number(num_str).unwrap().to_string()
            } else {
                num_str.to_string()
            }
        }
        Rule::VAR_NAME => {
            // Return placeholder - actual compilation tracks this
            format!("${}", pair.as_str())
        }
        _ => String::new(),
    }
}
