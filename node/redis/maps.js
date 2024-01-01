const redis = require('redis');

async function hashOperationsDemo() {
    const client = redis.createClient({
        url: 'redis://localhost:6379'
    });

    client.on('error', (err) => console.log('Redis Client Error', err));

    try {
        await client.connect();

        let userKey = 'user:1000';

        // HSET: Set individual fields in the hash
        await client.hSet(userKey, 'name', 'Alice');
        await client.hSet(userKey, 'age', '30');

        // HGET: Get an individual field from the hash
        let name = await client.hGet(userKey, 'name');
        console.log('Name:', name);

        // HGETALL: Get all fields and values from the hash
        let user = await client.hGetAll(userKey);
        console.log('User:', user);

        // HMSET: Set multiple fields in the hash (deprecated in favor of HSET)
        await client.hSet(userKey, { 'email': 'alice@example.com', 'city': 'Wonderland' });

        // HMGET: Get multiple fields from the hash
        let values = await client.hmGet(userKey, 'name', 'email', 'city');
        console.log('Multiple Fields:', values);

        // HDEL: Delete a field from the hash
        await client.hDel(userKey, 'age');

        // HEXISTS: Check if a field exists in the hash
        let exists = await client.hExists(userKey, 'age');
        console.log('Does age exist?', exists);

        // Show final state of the hash
        user = await client.hGetAll(userKey);
        console.log('Final User:', user);

    } catch (err) {
        console.error('Error:', err);
    } finally {
        await client.quit();
    }
}

hashOperationsDemo();

