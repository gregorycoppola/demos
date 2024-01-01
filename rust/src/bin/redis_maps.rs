extern crate redis;
use redis::{Commands, RedisResult};

fn main() -> RedisResult<()> {
    // Connect to the local Redis server
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con: redis::Connection = client.get_connection()?;

    let user_key = "user:1000";

    // HSET: Set individual fields in the hash
    let _: () = con.hset(user_key, "name", "Alice")?;
    let _: () = con.hset(user_key, "age", "30")?;

    // HGET: Get an individual field from the hash
    let name: String = con.hget(user_key, "name")?;
    println!("Name: {}", name);

    // HGETALL: Get all fields and values from the hash
    let user: std::collections::HashMap<String, String> = con.hgetall(user_key)?;
    println!("User: {:?}", user);

    // HSET: Set multiple fields in the hash (replaces HMSET)
    // Note: This feature might not be available in all versions of the redis crate.
    // If it's not available, you'll need to set each field individually.
    let _: () = con.hset_multiple(
        user_key,
        &[("email", "alice@example.com"), ("city", "Wonderland")],
    )?;

    // HMGET: Get multiple fields from the hash
    let values: Vec<Option<String>> = con.hget(user_key, &["name", "email", "city"][..])?;
    println!("Multiple Fields: {:?}", values);

    // HDEL: Delete a field from the hash
    let _: () = con.hdel(user_key, "age")?;

    // HEXISTS: Check if a field exists in the hash
    let exists: bool = con.hexists(user_key, "age")?;
    println!("Does age exist? {}", exists);

    // Show final state of the hash
    let user: std::collections::HashMap<String, String> = con.hgetall(user_key)?;
    println!("Final User: {:?}", user);

    Ok(())
}
