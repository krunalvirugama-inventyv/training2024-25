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
  } else {
    res.writeHead(404, { "Content-Type": "application/json" });
    res.end(JSON.stringify({ error: "Route not found" }));
  }
});

server.listen(process.env.PORT, () => {
  console.log(`Server running at Port ${process.env.PORT}`);
});
