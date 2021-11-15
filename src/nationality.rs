use postgres::{Client, NoTls, Error};
use dotenv::dotenv;
use std::env;

struct Nation {
    nationality: String,
    count: i64,
}

fn main() -> Result<(), Error> {
    dotenv().ok();
    let DB_USER = env::var("DB_USER").unwrap();
    let DB_PASSWORD = env::var("DB_PASSWORD").unwrap();
    let DB = env::var("DB").unwrap();

    let address = format!("postgresql://{}:{}@localhost/{}", DB_USER, DB_PASSWORD, DB);
    let mut client = Client::connect(&address, NoTls);
    
    Ok(())
}