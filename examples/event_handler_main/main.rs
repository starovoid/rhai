//! Implementation of the Event Handler With State Pattern - Main Style

#[cfg(feature = "no_function")]
pub fn main() {
    panic!("This example does not run under 'no_function'.")
}

#[cfg(not(feature = "no_function"))]
pub fn main() {
    use rhai::{CallFnOptions, Dynamic, Engine, Scope, AST};
    use std::io::{stdin, stdout, Write};
    use std::path::Path;

    const SCRIPT_FILE: &str = "event_handler_main/script.rhai";

    #[derive(Debug)]
    struct Handler {
        pub engine: Engine,
        pub scope: Scope<'static>,
        pub ast: AST,
    }

    fn print_scope(scope: &Scope) {
        for (i, (name, constant, value)) in scope.iter_raw().enumerate() {
            #[cfg(not(feature = "no_closure"))]
            let value_is_shared = if value.is_shared() { " (shared)" } else { "" };
            #[cfg(feature = "no_closure")]
            let value_is_shared = "";

            println!(
                "[{}] {}{}{} = {:?}",
                i + 1,
                if constant { "const " } else { "" },
                name,
                value_is_shared,
                *value.read_lock::<Dynamic>().unwrap(),
            )
        }
        println!();
    }

    println!("Events Handler Example - Main Style");
    println!("===================================");

    let mut input = String::new();

    // Read script file
    print!("Script file [{}]: ", SCRIPT_FILE);
    stdout().flush().expect("flush stdout");

    input.clear();

    stdin().read_line(&mut input).expect("read input");

    let path = match input.trim() {
        "" => SCRIPT_FILE,
        path => path,
    };

    // Create Engine
    let engine = Engine::new();

    // Create a custom 'Scope' to hold state
    let mut scope = Scope::new();

    // Add any system-provided state into the custom 'Scope'.
    // Constants can be used to optimize the script.
    scope.push_constant("MY_CONSTANT", 42_i64);

    // Compile the handler script.
    println!("> Loading script file: {path}");

    let ast = match engine.compile_file_with_scope(&mut scope, &Path::new(path)) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("! Error: {}", err);
            println!("Cannot continue. Bye!");
            return;
        }
    };

    println!("> Script file loaded.");
    println!();
    println!("quit      = exit program");
    println!("scope     = print scope");
    println!("event arg = run function with argument");
    println!();

    // Run the 'init' function to initialize the state, retaining variables.
    let options = CallFnOptions::new().eval_ast(false).rewind_scope(false);

    let result = engine.call_fn_with_options::<()>(options, &mut scope, &ast, "init", ());

    if let Err(err) = result {
        eprintln!("! {err}")
    }

    // Create handler instance
    let mut handler = Handler { engine, scope, ast };

    // Events loop
    loop {
        print!("event> ");
        stdout().flush().expect("flush stdout");

        // Read event
        input.clear();
        stdin().read_line(&mut input).expect("read input");

        let mut fields = input.trim().splitn(2, ' ');

        let event = fields.next().expect("event").trim();
        let arg = fields.next().unwrap_or("").to_string();

        // Process event
        match event {
            "quit" => break,

            "scope" => {
                print_scope(&handler.scope);
                continue;
            }

            // Map all other events to function calls
            _ => {
                let engine = &handler.engine;
                let scope = &mut handler.scope;
                let ast = &handler.ast;

                let result = engine.call_fn::<()>(scope, ast, event, (arg,));

                if let Err(err) = result {
                    eprintln!("! {err}")
                }
            }
        }
    }

    println!("Bye!");
}
