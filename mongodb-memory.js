#!/usr/bin/node

const { MongoMemoryServer } = require("mongodb-memory-server");

(async () => {
  if (process.arch === "arm64" && process.platform === "darwin") {
    process.env.MONGOMS_DOWNLOAD_URL = "https://fastdl.mongodb.org/osx/mongodb-macos-arm64-6.0.5.tgz"
  }
  // This will create an new instance of "MongoMemoryServer" and automatically start it
  const mongod = await MongoMemoryServer.create({
    instance: {
      port: 50846,
    },
    /*  binary: {
       version: '6.0.5',
       skipMD5: true,
     }, */
  });
  const uri = mongod.getUri();
  console.log(uri);
})();
