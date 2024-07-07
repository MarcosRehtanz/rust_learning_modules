use regex::Regex;
mod authentication;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("\nDid our date match? {}\n", re.is_match("2014-01-01"));

    let username = "jeremy";
    let password = "super-secret";
    let new_password = "even-more-secret";

    let mut user = authentication::User::new(&username, &password);

    println!("The username is: {}", user.get_username());
    println!(
        "Username: {}\nPassword: {}\nLogged: {}\n",
        String::from(username),
        String::from(password),
        user.login(&username, &password)
    );
    user.set_password("even-more-secret");

    println!(
        "Username: {}\nPassword: {}\nLogged: {}\n",
        String::from(username),
        String::from(password),
        user.login(&username, &password)
    );
    println!(
        "Username: {}\nPassword: {}\nLogged: {}\n",
        String::from(username),
        String::from(new_password),
        user.login(&username, &new_password)
    );
}
