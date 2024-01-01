extern crate redis;
use redis::{RedisResult, Commands};

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

    // // HGETALL: Get all fields and values from the hash
    // let user: redis::Hash<String, String> = con.hgetall(user_key).await?;
    // println!("User: {:?}", user);

    // // HSET: Set multiple fields in the hash (replaces HMSET)
    // let _: () = con.hset_multiple(user_key, &["email", "alice@example.com", "city", "Wonderland"]).await?;

    // // HMGET: Get multiple fields from the hash
    // let values: Vec<Option<String>> = con.hget(user_key, &["name", "email", "city"][..]).await?;
    // println!("Multiple Fields: {:?}", values);

    // // HDEL: Delete a field from the hash
    // let _: () = con.hdel(user_key, "age").await?;

    // // HEXISTS: Check if a field exists in the hash
    // let exists: bool = con.hexists(user_key, "age").await?;
    // println!("Does age exist? {}", exists);

    // // Show final state of the hash
    // let user: redis::Hash<String, String> = con.hgetall(user_key).await?;
    // println!("Final User: {:?}", user);

    Ok(())
}

