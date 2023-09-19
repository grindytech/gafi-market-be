#!/usr/bin/node

const { MongoMemoryServer } = require("mongodb-memory-server");

(async () => {
  if (process.arch === "arm64" && process.platform === "darwin") {
    process.env.MONGOMS_DOWNLOAD_URL = "https://fastdl.mongodb.org/osx/mongodb-macos-arm64-6.0.5.tgz"
  } else {
    process.env.MONGOMS_DOWNLOAD_URL = "https://fastdl.mongodb.org/linux/mongodb-linux-x86_64-ubuntu1804-6.0.5.tgz"
  }
  var args = process.argv.slice(2);
  const port = Number(args[0]);

  // This will create an new instance of "MongoMemoryServer" and automatically start it
  const mongod = await MongoMemoryServer.create({
    instance: {
      port: port || 50846,
    },
  });
  const uri = mongod.getUri();
})();
