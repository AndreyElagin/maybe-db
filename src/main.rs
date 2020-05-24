use std::io;
use std::process::exit;

fn main() {
    let mut input = String::new();
    loop {
        print_prompt();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.chars().next().unwrap() == '.' {
                    match do_meta_command(&input) {
                        MetaCommand::Success =>
                            println!("currently nothing to do"),
                        MetaCommand::Unrecognized(cmd) =>
                            println!("Unrecognized meta command: {}", cmd)
                    }
                } else {
                    let st = match prepare_statement(&input) {
                        (Some(st), PrepareResult::Success) =>
                            Ok(st),
                        (_, PrepareResult::Unrecognized(v)) =>
                            Err(format!("Unrecognized statement: {}", v)),
                        (_, _) =>
                            Err(format!("unknown error for input: {}", input))
                    };
                    match st {
                        Ok(v) => execute_statement(v),
                        Err(e) => println!("{}", e)
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
        input.clear()
    }
}

use std::io::Write;

fn print_prompt() {
    print!("db > ");
    let _ = io::stdout().flush();
}

enum MetaCommand {
    Success,
    Unrecognized(String),
}

enum StatementType {
    Insert,
    Select,
}

enum PrepareResult {
    Success,
    Unrecognized(String),
}

struct Statement {
    tp: StatementType
}

fn do_meta_command(input: &str) -> MetaCommand {
    match input {
        ".exit\n" => exit(0),
        i => MetaCommand::Unrecognized(i.to_string())
    }
}

fn prepare_statement(input: &str) -> (Option<Statement>, PrepareResult) {
    match input {
        i if i.len() < 6 => (None, PrepareResult::Unrecognized(i.to_string())),
        i if &i[..6] == "insert" =>
            (Some(Statement { tp: StatementType::Insert }), PrepareResult::Success),
        i if &i[..6] == "select" =>
            (Some(Statement { tp: StatementType::Select }), PrepareResult::Success),
        i => (None, PrepareResult::Unrecognized(i.to_string()))
    }
}

fn execute_statement(st: Statement) {
    match st.tp {
        StatementType::Select => println!("Select statement"),
        StatementType::Insert => println!("Insert statement")
    }
}
