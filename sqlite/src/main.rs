use std::io;

fn main() {
    get_credentials();
    display_dashboard();
    do_actions();

}

fn get_credentials() -> (String, String) {
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
        if validate_credentials(&username, &password) {
            return (username, password);
        }
    }
}

fn validate_credentials(username: &str, password: &str) -> bool {

}

fn display_dashboard(username: &str, password: &str, clearance: u8, department: &str) {

}
