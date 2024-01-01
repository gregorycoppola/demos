extern crate redis;
extern crate bincode;
extern crate serde;

use redis::{Commands, RedisResult, Client};
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    emails: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Company {
    name: String,
    address: String,
    employees: Vec<Person>,
}

fn store_to_redis(key: &str, company: &Company) -> RedisResult<()> {
    // Connect to the local Redis server
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    // Serialize the company object
    let serialized = serialize(company).unwrap();

    // Store serialized data in Redis
    con.set(key, serialized)?;

    Ok(())
}

fn retrieve_from_redis(key: &str) -> RedisResult<Company> {
    // Connect to the local Redis server
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    // Retrieve the serialized data from Redis
    let serialized: Vec<u8> = con.get(key)?;

    // Deserialize the data back into a Company object
    let company: Company = deserialize(&serialized[..]).unwrap();

    Ok(company)
}

fn main() -> RedisResult<()> {
    let company = Company {
        name: "Acme Corporation".to_string(),
        address: "123 Acme Way".to_string(),
        employees: vec![
            Person {
                name: "John Doe".to_string(),
                age: 30,
                emails: vec!["john@example.com".to_string()],
            },
            Person {
                name: "Jane Smith".to_string(),
                age: 28,
                emails: vec!["jane@example.com".to_string(), "jsmith@example.com".to_string()],
            },
        ],
    };

    let key = "company_data";

    // Store the company object to Redis
    store_to_redis(key, &company)?;

    // Retrieve the company object from Redis
    let retrieved_company = retrieve_from_redis(key)?;

    println!("Retrieved Company: {:?}", retrieved_company);

    Ok(())
}
