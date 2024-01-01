extern crate redis;
use redis::Commands;

fn main() -> redis::RedisResult<()> {
    // Connect to the local Redis server
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    // Set a key
    let _: () = con.set("my_key", 42)?;
    
    // Get the value of the key
    let my_val: isize = con.get("my_key")?;
    println!("Got value from Redis: {}", my_val);

    Ok(())
}

