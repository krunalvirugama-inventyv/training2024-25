const express = require("express");
const fs = require("fs");
const path = require("path");
const cors = require("cors");
const bodyParser = require("body-parser");

const { connectDB } = require("./dbConfig/db");

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(cors());
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

// Save Report to TiDB
const saveReportToTiDB = async (reportData) => {
  const connection = await connectDB();
  console.log("Saving report to TiDB...");
  
  try {
    const [result] = await connection.execute(
      "INSERT INTO reports (report, created_at) VALUES (?, ?)",
      [JSON.stringify(reportData), new Date()]
    );
    
    console.log("Report saved successfully! Inserted ID:", result.insertId);
    return result.insertId;
  } catch (error) {
    console.error("Error saving report to TiDB:", error);
    throw error;
  } finally {
    await connection.end();
  }
};

// Fetch All Reports
const fetchReportsFromTiDB = async () => {
  const connection = await connectDB();
  
  try {
    const [rows] = await connection.execute("SELECT * FROM reports ORDER BY created_at DESC");
    return rows;
  } catch (error) {
    console.error("Error fetching reports:", error);
    throw error;
  } finally {
    await connection.end();
  }
};

// Fetch Latest Report
const fetchLatestReportFromTiDB = async () => {
  const connection = await connectDB();
  
  try {
    const [rows] = await connection.execute("SELECT * FROM reports ORDER BY created_at DESC LIMIT 1");
    return rows[0];
  } catch (error) {
    console.error("Error fetching latest report:", error);
    throw error;
  } finally {
    await connection.end();
  }
};

// Routes

// 📌 Save Report from JSON File
app.post("/save-report", async (req, res) => {
  try {
    const filePath = path.join(__dirname, "cucumber_report/cucumber-report.json");
    const reportData = JSON.parse(fs.readFileSync(filePath, "utf8"));

    await saveReportToTiDB(reportData);
    
    res.status(201).json({ message: "Report saved successfully!" });
  } catch (error) {
    console.error("Error saving report:", error);
    res.status(500).json({ error: "Internal Server Error" });
  }
});

// 📌 Fetch All Reports
app.get("/reports", async (req, res) => {
  try {
    const reports = await fetchReportsFromTiDB();
    res.status(200).json({ reports });
  } catch (error) {
    console.error("Error fetching reports:", error);
    res.status(500).json({ error: "Internal Server Error" });
  }
});

// 📌 Fetch Latest Report
app.get("/latest-report", async (req, res) => {
  try {
    const latestReport = await fetchLatestReportFromTiDB();

    if (!latestReport) {
      return res.status(404).json({ error: "No reports found" });
    }

    res.status(200).json({ latestReport });
  } catch (error) {
    console.error("Error fetching latest report:", error);
    res.status(500).json({ error: "Internal Server Error" });
  }
});

// Handle 404 Routes
app.use((req, res) => {
  res.status(404).json({ error: "Route not found" });
});

// Start Server
app.listen(PORT, () => {
  console.log(`🚀 Server running on port ${PORT}`);
});
