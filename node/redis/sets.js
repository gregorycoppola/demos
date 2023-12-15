const redis = require('redis');

async function setOperationsDemo() {
    const client = redis.createClient({
        url: 'redis://localhost:6379'
    });

    client.on('error', (err) => console.log('Redis Client Error', err));

    try {
        await client.connect();

        // SADD: Add members to sets
        // Add individual elements to set1
        await client.sAdd('set1', 'a');
        await client.sAdd('set1', 'b');
        await client.sAdd('set1', 'c');

        // Add individual elements to set2
        await client.sAdd('set2', 'b');
        await client.sAdd('set2', 'c');
        await client.sAdd('set2', 'd');

        // SMEMBERS: Get all members of a set
        let set1Members = await client.sMembers('set1');
        console.log('Members of set1:', set1Members);

        // SISMEMBER: Check if 'a' is a member of set1
        let isMember = await client.sIsMember('set1', 'a');
        console.log('Is \'a\' a member of set1?', isMember);

        // SREM: Remove a member from set1
        await client.sRem('set1', 'a');
        set1Members = await client.sMembers('set1');
        console.log('Members of set1 after removing \'a\':', set1Members);

        // SUNION: Union of set1 and set2
        let union = await client.sUnion('set1', 'set2');
        console.log('Union of set1 and set2:', union);

        // SINTER: Intersection of set1 and set2
        let intersection = await client.sInter('set1', 'set2');
        console.log('Intersection of set1 and set2:', intersection);

        // SDIFF: Difference between set1 and set2
        let difference = await client.sDiff('set1', 'set2');
        console.log('Difference between set1 and set2:', difference);

    } catch (err) {
        console.error('Error:', err);
    } finally {
        // Close the connection
        await client.quit();
    }
}

setOperationsDemo();

