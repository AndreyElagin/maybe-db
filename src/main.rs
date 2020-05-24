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
use std::fmt::Formatter;

fn print_prompt() {
    print!("db > ");
    let _ = io::stdout().flush();
}

const COLUMN_USERNAME_SIZE: u32 = 32;
const COLUMN_EMAIL_SIZE: u32 = 255;

const ID_SIZE: u32 = 4;
const USERNAME_SIZE: u32 = 32;

const EMAIL_SIZE: u32 = 255;

const ID_OFFSET: u32 = 0;
const USERNAME_OFFSET: u32 = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: u32 = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE: u32 = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

const PAGE_SIZE: u32 = 4096;
const TABLE_MAX_PAGES: u32 = 100;
const ROWS_PER_PAGE: u32 = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: u32 = ROWS_PER_PAGE * TABLE_MAX_PAGES;

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
    SyntaxError,
    Unrecognized(String),
}

struct Statement {
    tp: StatementType,
    // row_to_insert: Row
}

struct Row {
    id: u32,
    username: [char; COLUMN_USERNAME_SIZE as usize],
    email: [char; COLUMN_EMAIL_SIZE as usize],
}

use std::fmt;

impl fmt::Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut uname = String::from("");
        for i in 0usize..USERNAME_SIZE as usize {
            if self.username[i].is_alphanumeric() {
                todo!("doesn't work");
                uname.push(self.username[i])
            }
        }
        let mut email = String::from("");
        for i in 0usize..EMAIL_SIZE as usize {
            if self.username[i].is_alphanumeric() {
                todo!("doesn't work");
                email.push(self.email[i])
            }
        }

        write!(f, "Row id: {}, username: {}, email: {}", self.id, uname, email)
    }
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
