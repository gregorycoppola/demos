const redis = require('redis');

async function main() {
    // Create a Redis client
    const client = redis.createClient({
        url: 'redis://localhost:6379' // This is the default URL for a local Redis server
    });

    client.on('error', (err) => console.log('Redis Client Error', err));

    try {
        // Connect to Redis
        await client.connect();

        // Setting a key
        await client.set('mykey', 'Hello, Redis!');
        console.log('Set: Hello, Redis!');

        // Getting a key
        const value = await client.get('mykey');
        console.log('Get:', value);
    } catch (err) {
        console.error(err);
    } finally {
        // Close the connection
        await client.quit();
    }
}

main();

