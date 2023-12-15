const redis = require('redis');

async function listDemo() {
    const client = redis.createClient({
        url: 'redis://localhost:6379'
    });

    client.on('error', (err) => console.log('Redis Client Error', err));

    try {
        await client.connect();

        let listKey = 'myList';

        // LPUSH - Add elements to the beginning of the list
        await client.lPush(listKey, 'apple', 'banana');
        console.log('LPUSH: Added apple, banana');

        // RPUSH - Add elements to the end of the list
        await client.rPush(listKey, 'cherry');
        console.log('RPUSH: Added cherry');

        // LRANGE - Get a range of elements
        let range = await client.lRange(listKey, 0, -1);
        console.log('LRANGE 0 -1:', range);

        // LPOP - Remove and get the first element
        let firstElement = await client.lPop(listKey);
        console.log('LPOP:', firstElement);

        // RPOP - Remove and get the last element
        let lastElement = await client.rPop(listKey);
        console.log('RPOP:', lastElement);

        // LLEN - Get the length of the list
        let length = await client.lLen(listKey);
        console.log('LLEN:', length);

        // LINDEX - Get an element by its index
        let elementAtIndex = await client.lIndex(listKey, 0);
        console.log('LINDEX 0:', elementAtIndex);

        // LSET - Set the value of an element
        await client.lSet(listKey, 0, 'grape');
        console.log('LSET index 0 to grape');

        // LTRIM - Trim the list
        await client.lTrim(listKey, 0, 1);
        console.log('LTRIM to first two elements');

        // Show final state of the list
        range = await client.lRange(listKey, 0, -1);
        console.log('Final List:', range);

    } catch (err) {
        console.error('Error:', err);
    } finally {
        await client.quit();
    }
}

listDemo();

