use chrono::prelude::*;
use rusqlite::Connection;
use rusqlite::Transaction;
use std::io::{self, Write};

#[derive(Debug)]
struct User {
    name: String,
    password: String,
    date_entered: String,
    department: String,
    clearance: u8,
}

struct Request {
    incident_id: u32,
    summary: String,
    description: String,
    date_entered: u64,
    department: String,
    clearance: u8,
}

fn main() {
    let mut conn = Connection::open("new.db").expect("Communication with database failed");
    let tx: Transaction = conn
        .transaction()
        .expect("Communication with database failed");

    tx.execute("CREATE TABLE IF NOT EXISTS credentials (username MEDIUMTEXT NOT NULL, password MEDIUMTEXT NOT NULL, date_entered TEXT NOT NULL, department TEXT, clearance UNSIGNED TINYINT, PRIMARY KEY (username))", []).expect("Failed to create Table");
    tx.execute("CREATE TABLE IF NOT EXISTS requests (incident_id MEDIUMINT UNSIGNED NOT NULL, summary TEXT, description TEXT NOT NULL, date_entered BIGINT UNSIGNED NOT NULL, department TEXT, request_source TEXT, category TEXT, color TEXT, reason_red TEXT, date_completed TEXT, date_expected TEXT, date_retired TEXT, PRIMARY KEY (incident_id))", []).expect("Failed to create Table");

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
    add_user(&*username, &*password, "Admin", "3", &tx);

    tx.commit().expect("Communication with database failed");
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

fn add_user(username: &str, password: &str, department: &str, clearance: &str, tx: &Transaction) {
    let date_entered = get_start_date();
    if !contains_credential(username, &tx) {
        tx.execute("insert into credentials (username, password, date_entered, department, clearance) values (?1, ?2, ?3, ?4, ?5)", [username, password, &date_entered, department, clearance]).expect("Communication with database failed");
    } else {
        println!("Database already contains that user profile")
    }
}

fn get_start_date() -> String {
    format!("{}", Utc::now())
}
