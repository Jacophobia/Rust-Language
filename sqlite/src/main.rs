use std::io;
use std::io::Write;
use rusqlite::{params, Connection, Result, Transaction};
use chrono::prelude::*;

#[derive(Debug)]
struct User {
    name: String,
    password: String,
    date_entered: String,
    department: String,
    clearance: u8,
}

#[derive(Debug)]
struct Request {
    incident_id: u32,
    summary: String,
    description: String,
    date_entered: u64,
    department: String,
    clearance: u8,
}

fn main() {
    let mut conn = Connection::open("data/requests.db").expect("Unable to open database.");
    let tx: Transaction = conn.transaction().expect("Transaction failed.");
    tx.execute("CREATE TABLE IF NOT EXISTS credentials (username MEDIUMTEXT NOT NULL, password MEDIUMTEXT NOT NULL, date_entered BIGINT UNSIGNED NOT NULL, department TEXT, clearance UNSIGNED TINYINT, PRIMARY KEY (username))", []);
    let (username, clearance, department) = get_login(&tx);
    tx.execute(&*format!("CREATE TABLE IF NOT EXISTS {} (incident_id MEDIUMINT UNSIGNED NOT NULL, summary TEXT, description TEXT NOT NULL, date_entered BIGINT UNSIGNED NOT NULL, department TEXT, request_source TEXT, category TEXT, color TEXT, reason_red TEXT, date_completed TEXT, date_expected TEXT, date_retired TEXT, PRIMARY KEY (incident_id))", department), []);
    display_dashboard(&username, &clearance, &department, &tx);
    do_actions(&username, &clearance, &department, &tx);
    save_changes(&tx);
}

fn get_login(tx: &Transaction) -> (String, u8, String) {
    let mut username;
    let mut password;
    loop {
        username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Unable to read input");
        password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Unable to read input");
        if validate_credentials(&username, &password, &tx) {
            (username, get_clearance(&username, &tx), get_department(&username))
        }
    }
}

fn validate_credentials(username: &str, password: &str, tx: &Transaction) -> bool {
    // Access the sql database and see if the username & password match.

    if contains_credential(username, &tx) {
        return get_credentials(username, &tx).expect("Unable to retrieve credentials").password == password
    }
    else {
        false
    }
}

fn get_clearance(username: &str, tx: &Transaction) -> u8 {
    // Access the sql database and get the clearance level of the user.
    if contains_credential(username, &tx) {
        get_credentials(username, &tx).expect("Unable to get clearance level of user.").clearance
    }
    else {
        let default: u8 = 0;
        default
    }
}

fn get_department(username: &str) -> String {
    // Access the sql database and get the department of the user.
    if contains_credential(username, &tx) {
        get_credentials(username, &tx).expect("Unable to get clearance level of user.").department
    }
    else {
        let default: String = "".to_string();
        default
    }
}

fn display_dashboard(username: &str, clearance: &u8, department: &str, tx: &Transaction) {
    // Access the sql database and get all relevant data for the user.

}

fn do_actions(username: &str, clearance: &u8, department: &str, tx: &Transaction) {
    loop {
        let mut done = false;
        if clearance == 0 {
            display_limited_actions();
            let action = get_limited_actions();
            match action {
                1 => make_account_request(&username),
                2 => return,
                _ => panic!("Encountered an unexpected action."),
            }
            done = true;
        }
        while !done {
            display_actions();
            let action = get_action();
            match action {
                1 => add_request(department),
                2 => update_request(),
                3 => add_user(&tx),
                4 => return,
                _ => continue,
            };
        }
    }
}

fn display_limited_actions() {
    println!("Please select an action:");
    println!("1 - Request an account upgrade from a supervisor");
    println!("2 - Exit Program")
}

fn get_limited_actions() -> u8 {
    loop {
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Unable to read input");
        let action: u8 = match action.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Invalid entry");
                continue;
            },
        };
        match action {
            1 => return action,
            2 => return action,
            _ => {
                println!("Error: Invalid entry");
                continue;
            },
        };
    }
}

fn make_account_request(username: &str) {
    // Add the username to a table in the database for account requests.
    // Maybe let them enter a note.
}

fn display_actions() {
    // display the actions available to the user.
    println!("Please select an action:");
    println!("1 - Add Request");
    println!("2 - Update Request");
    println!("3 - Add User");
    println!("4 - Exit Program");
}

fn get_action() -> u8 {
    loop {
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Unable to read input");
        let action: u8 = match action.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Invalid entry");
                continue;
            },
        };
        match action {
            1 => return action,
            2 => return action,
            3 => return action,
            4 => return action,
            _ => {
                println!("Error: Invalid entry");
                continue;
            },
        };
    }
}

// add_request
fn add_user(tx: &Transaction) {
    let mut username = String::new();
    let mut password = String::new();
    let mut valid_username = false;
    while !valid_username {
        print!("Enter a username that you want to use for authentication\n> ");
        io::stdout().flush().expect("Failed to flush stdout.");
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read input");
        if !contains_credential(&*username, &tx) {
            valid_username = true;
            username = username.trim().parse().expect("Failed to parse username.");
        } else {
            println!("That username already exists.");
        }
    }
    let mut valid_password = false;
    while !valid_password {
        print!("Enter the password that you want to use for authentication\n> ");
        io::stdout().flush().expect("Failed to flush stdout.");
        let size = io::stdin()
            .read_line(&mut password)
            .expect("Failed to read input");
        if size >= 8 {
            valid_password = true;
            password = password.trim().parse().expect("Failed to parse password");
        } else {
            println!("That password is too short.");
        }
    }
    _add_user(&*username, &*password, "Admin", "3", &tx);

}

fn _add_user(username: &str, password: &str, department: &str, clearance: &str, tx: &Transaction) {
    let date_entered = get_start_date();
    if !contains_credential(username, &tx) {
        tx.execute("insert into credentials (username, password, date_entered, department, clearance) values (?1, ?2, ?3, ?4, ?5)", [username, password, &date_entered, department, clearance]).expect("Communication with database failed");
    } else {
        println!("Database already contains that user profile")
    }
}

fn contains_credential(username: &str, tx: &Transaction) -> bool {
    // SELECT * FROM credentials WHERE username='{}', username
    // values:
    //     1: username,   2: password,  3: date_entered,
    //     4: department, 5: clearance, 6: all

    match get_credentials(username, &tx) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn get_credentials(username: &str, tx: &Transaction) -> Result<User, String> {
    let mut stmt = tx
        .prepare(&*format!(
            "SELECT * FROM credentials WHERE username='{}'",
            username
        ))
        .expect("Unable to prepare retrieval");
    let rows = stmt
        .query_map([], |row| {
            Ok(User {
                name: row.get(0).expect("Unable to extract username"),
                password: row.get(1).expect("Unable to extract password"),
                date_entered: row.get(2).expect("Unable to extract date entered"),
                department: row.get(3).expect("Unable to extract department"),
                clearance: row.get(4).expect("Unable to extract clearance"),
            })
        })
        .expect("Unable to extract data");
    for row in rows {
        let user = row.expect("Unable to read row data");
        if user.name == username.to_string() {
            return Ok(user);
        }
    }
    Err("Does not contain username"
        .parse()
        .expect("Unable to parse value"))
}

fn save_changes(tx: &Transaction) {
    // Save the changes to the database.
    tx.commit().expect("Failed to save changes");
}