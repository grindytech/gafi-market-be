#!/usr/bin/node

const { MongoMemoryServer } = require("mongodb-memory-server");

(async () => {
  if (process.arch === "arm64" && process.platform === "darwin") {
    process.env.MONGOMS_DOWNLOAD_URL = "https://fastdl.mongodb.org/osx/mongodb-macos-arm64-6.0.5.tgz"
  }
  var args = process.argv.slice(2);

  // This will create an new instance of "MongoMemoryServer" and automatically start it
  const mongod = await MongoMemoryServer.create();
  const uri = mongod.getUri();
  console.log(uri)

  if (args[0]) {
    const timeOut = Number(args[0]);
    setTimeout(() => {
      mongod.stop()
    }, timeOut);
  }
})();
