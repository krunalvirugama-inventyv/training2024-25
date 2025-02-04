
const couchbase = require("couchbase");
require("dotenv").config();

const connectDB = async () => {
  try {

    const cluster = await couchbase.connect(process.env.COUCHBASE_URI, {
      username: process.env.COUCHBASE_USERNAME,
      password: process.env.COUCHBASE_PASSWORD,
    });

    const bucket = cluster.bucket(process.env.COUCHBASE_BUCKET);
    // const collection = bucket.defaultCollection();
    const collection = bucket.scope("_default").collection("reports");



    console.log(`Couchbase connected to: ${process.env.COUCHBASE_URI}`);
    
  
    return { cluster, collection };
  
  } catch (error) {
    console.error(`Error connecting to Couchbase : ${error.message}`);
    process.exit(1);
  }
};

module.exports = { connectDB };
