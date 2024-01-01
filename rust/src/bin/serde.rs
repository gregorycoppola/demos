use serde::{Serialize, Deserialize};

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

fn main() {
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

    // Serialize using bincode
    let serialized = bincode::serialize(&company).unwrap();

    println!("Serialized Company: {:?}", serialized);

    // Step 4: Deserialize
    let deserialized: Company = bincode::deserialize(&serialized).unwrap();

    println!("Deserialized Company: {:?}", deserialized);
}
