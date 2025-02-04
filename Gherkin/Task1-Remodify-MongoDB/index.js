const http = require("http");
const fs = require("fs");
const path = require("path");
const { connectDB } = require("./dbConfig/db");
const Report = require("./modal/Report");

connectDB();

const server = http.createServer(async (req, res) => {
  if (req.method === "POST" && req.url === "/save-report") {
    try {
      // Read the JSON file
      const filePath = path.join(__dirname, "cucumber_report/cucumber-report.json");
      const reportData = JSON.parse(fs.readFileSync(filePath, "utf8"));

      // Save data to MongoDB
      const newReport = new Report({ report: reportData });
      await newReport.save();

      res.writeHead(201, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ message: "Report saved successfully!" }));
    } catch (error) {
      console.error("Error saving report:", error);
      res.writeHead(500, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ error: "Internal Server Error" }));
    }
  } 
  else if (req.method === "GET" && req.url === "/report") {
    try {
      // Fetch all reports sorted in descending order
      const reports = await Report.find().sort({ createdAt: -1 });

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
      // Fetch the latest report
      const latestReport = await Report.findOne().sort({ createdAt: -1 });

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
