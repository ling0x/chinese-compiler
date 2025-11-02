// main.rs
use chinese_compiler::{ChineseLangParser, Rule};
use inkwell::context::Context;
use pest::Parser;
use std::fs;
use std::process::Command;

mod codegen;
use codegen::Compiler;

fn main() {
    let source = fs::read_to_string("programs/poem.zh").expect("无法读取文件");

    // Parse the Chinese code
    let pairs = ChineseLangParser::parse(Rule::PROGRAM, &source).expect("解析错误");

    // Create LLVM context and compiler
    let context = Context::create();
    let mut compiler = Compiler::new(&context, "chinese_program");

    compiler.declare_printf();
    compiler.create_main_function();

    // Compile to LLVM IR
    for pair in pairs {
        compile_statement(pair, &mut compiler);
    }

    compiler.finish_main();

    // Output LLVM IR (human-readable)
    compiler.write_llvm_ir("output.ll");
    println!("✓ Generated LLVM IR: output.ll");

    // Output object file
    compiler.write_object_file("output.o");
    println!("✓ Generated object file: output.o");

    // Link to executable using clang
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
    // Simplified - adapt your existing parser logic
    match pair.as_rule() {
        Rule::VAR_DECL => {
            // Extract var name and value, compile to LLVM
            compiler.compile_var_decl("x", 42);
        }
        Rule::PRINT_STMT => {
            compiler.compile_print("x");
        }
        _ => {}
    }
}
