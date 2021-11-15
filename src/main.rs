use std::{collections::HashMap, env};
use dotenv::dotenv;
use postgres::{Client, NoTls, Error};

struct Author {
    _id: i32,
    name: String,
    country: String
}

fn getAddress() -> Result<String,()> {
    dotenv().ok();
    let DB_USER = env::var("DB_USER").unwrap();
    let DB_PASSWORD = env::var("DB_PASSWORD").unwrap();
    let DB = env::var("DB").unwrap();

    let address = format!("postgresql://{}:{}@localhost/{}", DB_USER, DB_PASSWORD, DB);
    Ok(address)
}

fn main() -> Result<(), Error> {
    let address = getAddress().unwrap();
    let mut client = Client::connect(&address, NoTls)?;

    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    for (key_name, value_country) in &authors {
        let author = Author {
            _id: 0,
            name: key_name.to_string(),
            country: value_country.to_string()
        };

        client.execute(
    "INSERT INTO author (name, country) VALUES ($1, $2)", 
        &[&author.name, &author.country]
        )?;
    }

    for row in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }


    Ok(())
}