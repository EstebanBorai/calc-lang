use super::{run_interactive, SymbolTable};

pub fn exec() {
    println!("Calc Language - Interactive");

    let mut variables = SymbolTable::new();

    loop {
        let input = prompt();

        if input.len() == 0 {
            eprintln!(
                r#"Invalid input received.\n
        Expected one of the following commands:\n\n
        - q: Quit current REPL instance\n
        - c: Clear variables\n
        - v: Print context variables"#
            );

            continue;
        }

        match input.trim() {
            "q" => {
                eprintln!("Exiting Calc Interactive");

                break;
            }
            "c" => {
                variables = SymbolTable::new();
                eprintln!("Variables cleared");

                continue;
            }
            "v" => {
                variables.print();

                continue;
            }
            source => {
                // Any other input is treated as a calc program
                if let Err(error_message) = run_interactive(&mut variables, source) {
                    eprintln!("{}", error_message);
                }
            }
        }
    }
}

fn prompt() -> String {
    let mut buf = String::new();

    eprint!("> ");

    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read from stdin");

    buf
}
