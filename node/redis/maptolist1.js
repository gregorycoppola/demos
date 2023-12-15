const redis = require('redis');

async function listDemo() {
    const client = redis.createClient({
        url: 'redis://localhost:6379'
    });

    client.on('error', (err) => console.log('Redis Client Error', err));

    try {
        await client.connect();

        await client.del('list:Alice');
await client.del('list:Bob');

        // Create lists for users
        await client.rPush('list:Alice',  'item1');
        await client.rPush('list:Alice', 'item2');
        await client.rPush('list:Bob', 'item3', 'item4');

        // Map user names to their list keys in a hash
        await client.hSet('userLists', 'Alice', 'list:Alice');
        await client.hSet('userLists', 'Bob', 'list:Bob');

        // Retrieve a user's list key and then get the list
        let aliceListKey = await client.hGet('userLists', 'Alice');
        let aliceList = await client.lRange(aliceListKey, 0, -1);

        console.log('Alice\'s List:', aliceList);


    } catch (err) {
        console.error('Error:', err);
    } finally {
        await client.quit();
    }
}

listDemo();

