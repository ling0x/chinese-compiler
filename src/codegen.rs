use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use std::collections::HashMap;

pub struct Compiler<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        Compiler {
            context,
            module,
            builder,
            variables: HashMap::new(),
        }
    }

    // Declare printf for output
    pub fn declare_printf(&self) {
        let i8_type = self.context.i8_type();
        let i8_ptr_type = i8_type.ptr_type(AddressSpace::default());
        let printf_type = self.context.i32_type().fn_type(
            &[i8_ptr_type.into()],
            true, // variadic
        );
        self.module.add_function("printf", printf_type, None);
    }

    // Compile: 变量 x = 42;
    pub fn compile_var_decl(&mut self, var_name: &str, value: i64) {
        let i64_type = self.context.i64_type();
        let alloca = self.builder.build_alloca(i64_type, var_name).unwrap();
        let const_val = i64_type.const_int(value as u64, false);
        self.builder.build_store(alloca, const_val).unwrap();
        self.variables.insert(var_name.to_string(), alloca);
    }

    // Compile: 输出 x;
    pub fn compile_print(&self, var_name: &str) {
        let printf = self.module.get_function("printf").unwrap();

        // Create format string "%lld\n"
        let format_str = self
            .builder
            .build_global_string_ptr("%lld\n", "fmt")
            .unwrap();

        // Load variable value
        let var_ptr = self.variables.get(var_name).unwrap();
        let value = self
            .builder
            .build_load(self.context.i64_type(), *var_ptr, var_name)
            .unwrap();

        // Call printf
        self.builder
            .build_call(
                printf,
                &[format_str.as_pointer_value().into(), value.into()],
                "printf_call",
            )
            .unwrap();
    }

    // Create main function wrapper
    pub fn create_main_function(&self) -> FunctionValue<'ctx> {
        let i32_type = self.context.i32_type();
        let main_type = i32_type.fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_type, None);
        let entry = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry);
        main_fn
    }

    pub fn finish_main(&self) {
        let i32_type = self.context.i32_type();
        let zero = i32_type.const_int(0, false);
        self.builder.build_return(Some(&zero)).unwrap();
    }

    // Output LLVM IR to file
    pub fn write_llvm_ir(&self, path: &str) {
        self.module.print_to_file(path).unwrap();
    }

    // Output object file
    pub fn write_object_file(&self, path: &str) {
        use inkwell::targets::{
            CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
        };

        Target::initialize_native(&InitializationConfig::default()).unwrap();
        let target_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&target_triple).unwrap();
        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                inkwell::OptimizationLevel::Default,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        target_machine
            .write_to_file(&self.module, FileType::Object, path.as_ref())
            .unwrap();
    }
}
