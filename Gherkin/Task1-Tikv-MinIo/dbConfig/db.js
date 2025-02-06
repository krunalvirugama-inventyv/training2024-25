const mysql = require("mysql2/promise");
require("dotenv").config();

const connectDB = async () => {
  try {
    const connection = await mysql.createConnection({
      host: process.env.TIDB_HOST || "127.0.0.1",
      port: process.env.TIDB_PORT || 4000,
      user: process.env.TIDB_USER || "root",
      password: process.env.TIDB_PASSWORD || "",
      database: process.env.TIDB_DATABASE || "reports_db",
    });

    console.log("Connected to TiDB!");
    return connection;
  } catch (error) {
    console.error("Error connecting to TiDB:", error);
    process.exit(1);
  }
};

module.exports = { connectDB };