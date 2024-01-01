extern crate redis;
use redis::{Commands, RedisResult};

fn main() -> RedisResult<()> {
    // Connect to the local Redis server
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con: redis::Connection = client.get_connection()?;

    let set_key = "user_set";

    // SADD: Add elements to the set
    let _: () = con.sadd(set_key, "Alice")?;
    let _: () = con.sadd(set_key, "Bob")?;
    let _: () = con.sadd(set_key, "Eve")?;

    // SMEMBERS: Get all members of the set
    let members: Vec<String> = con.smembers(set_key)?;
    println!("Set members: {:?}", members);

    // SISMEMBER: Check if a member is in the set
    let is_member: bool = con.sismember(set_key, "Alice")?;
    println!("Is Alice in the set? {}", is_member);

    // SREM: Remove members from the set
    let _: () = con.srem(set_key, "Eve")?;

    // Show the current state of the set
    let current_members: Vec<String> = con.smembers(set_key)?;
    println!("Current set members: {:?}", current_members);

    // SCARD: Get the number of members in the set
    let set_size: isize = con.scard(set_key)?;
    println!("Number of members in the set: {}", set_size);

    Ok(())
}
