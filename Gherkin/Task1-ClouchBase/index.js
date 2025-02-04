const http = require("http");
const fs = require("fs");
const path = require("path");
const { connectDB } = require("./dbConfig/db");
const couchbase = require("couchbase");

const saveReportToCouchbase = async (reportData) => {
  const { collection } = await connectDB();
  const reportId = `report_${Date.now()}`; // Generate a unique report ID

  try {
    await collection.upsert(reportId, { report: reportData, createdAt: new Date() });
    console.log("Report saved successfully!");  
  } catch (error) {
    console.error("Error saving report to Couchbase:", error);
    throw error;
  }
};

const fetchReportsFromCouchbase = async () => {

  const { cluster } = await connectDB();

  try {
    const query = `SELECT * FROM \`${process.env.COUCHBASE_BUCKET}\`._default.reports ORDER BY createdAt DESC`;
    // Adjust bucket name if necessary
    const result = await cluster.query(query);
    return result.rows;
  } catch (error) {
    console.error("Error fetching reports:", error);
    throw error;
  }
};

const fetchLatestReportFromCouchbase = async () => {
  const { cluster } = await connectDB();
  try {
    // Using the bucket name dynamically from environment variable
    const query = `SELECT * FROM \`${process.env.COUCHBASE_BUCKET}\`._default.reports ORDER BY createdAt DESC LIMIT 1`;

    const result = await cluster.query(query);
    return result.rows[0]; // Get the latest report
  } catch (error) {
    console.error("Error fetching latest report:", error);
    throw error;
  }
};

const server = http.createServer(async (req, res) => {
  if (req.method === "POST" && req.url === "/save-report") {
    try {
   
      const filePath = path.join(__dirname, "cucumber_report/cucumber-report.json");
      const reportData = JSON.parse(fs.readFileSync(filePath, "utf8"));

      await saveReportToCouchbase(reportData);

      res.writeHead(201, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ message: "Report saved successfully!" }));
    } catch (error) {
      console.error("Error saving report:", error);
      res.writeHead(500, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ error: "Internal Server Error" }));
    }
  } 
  else if (req.method === "GET" && req.url === "/reports") {
    try {
      // Fetch all reports sorted in descending order
      const reports = await fetchReportsFromCouchbase();

      res.writeHead(200, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ reports }));
    } catch (error) {
      console.error("Error fetching reports:", error);
      res.writeHead(500, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ error: "Internal Server Error" }));
    }
  } 
  else if (req.method === "GET" && req.url === "/latest-report") {
    try {
   
      const latestReport = await fetchLatestReportFromCouchbase();

      if (!latestReport) {
        res.writeHead(404, { "Content-Type": "application/json" });
        return res.end(JSON.stringify({ error: "No reports found" }));
      }

      res.writeHead(200, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ latestReport }));
    } catch (error) {
      console.error("Error fetching latest report:", error);
      res.writeHead(500, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ error: "Internal Server Error" }));
    }
  } 
  else {
    res.writeHead(404, { "Content-Type": "application/json" });
    res.end(JSON.stringify({ error: "Route not found" }));
  }
});

const PORT = process.env.PORT || 5000;
server.listen(PORT, () => {
  console.log(`Server running at Port ${PORT}`);
});
