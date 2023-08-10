#!/usr/bin/node

const { MongoMemoryServer } = require("mongodb-memory-server");

(async () => {
  // This will create an new instance of "MongoMemoryServer" and automatically start it
  const mongod = await MongoMemoryServer.create({
    instance: {
      port: 50846,
    },
  });
  const uri = mongod.getUri();
  console.log(uri);
})();
