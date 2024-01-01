extern crate redis;
use redis::{Commands, RedisResult};

fn main() -> RedisResult<()> {
    // Connect to the local Redis server
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con: redis::Connection = client.get_connection()?;

    let list_key = "user_list";

    // RPUSH: Push elements to the end of the list
    let _: () = con.rpush(list_key, "Alice")?;
    let _: () = con.rpush(list_key, "Bob")?;

    // LPUSH: Push elements to the start of the list
    let _: () = con.lpush(list_key, "Eve")?;

    // LRANGE: Get a range of elements from the list
    let list_elements: Vec<String> = con.lrange(list_key, 0, -1)?;
    println!("List elements: {:?}", list_elements);

    // LPOP: Remove and get the first element of the list
    let first_element: Option<String> = con.lpop(list_key, None)?;
    println!("Popped first element: {:?}", first_element);

    // RPOP: Remove and get the last element of the list
    let last_element: Option<String> = con.rpop(list_key, None)?;
    println!("Popped last element: {:?}", last_element);

    // Show the current state of the list
    let current_list: Vec<String> = con.lrange(list_key, 0, -1)?;
    println!("Current list: {:?}", current_list);

    Ok(())
}
