# Task1: Test Automation Project with MongoDB 🚀

## 📋 What is Task1?

A test automation project combining **Playwright** and **Cucumber.js** for end-to-end testing, now enhanced with **MongoDB** integration for report storage and retrieval.

### Key Features:

- **Playwright**: Browser automation and UI testing.
- **Cucumber.js**: BDD tests using Gherkin syntax.
- **Custom Reports**: Script to merge/format results via `generateReport.js`.
- **MongoDB Integration**: Store and retrieve reports from the database.

## 🛠️ Prerequisites

- Node.js v16+ ([Download](https://nodejs.org/))
- npm (comes with Node.js)
- MongoDB ([Download](https://www.mongodb.com/)) or MongoDB Atlas
- Modern browser (Chromium, Firefox, or WebKit)

## 📂 Project Structure

```
TASK1/
├── cucumber_report/            # Cucumber reports generated after running tests
│   ├── cucumber_report.html    # HTML report for Cucumber tests
│   ├── cucumber_report.json    # JSON report for Cucumber tests
│
├── dbConfig/                   # Database configuration folder
│   ├── db.js                   # MongoDB connection setup
│
├── features/                   # Cucumber test features and step definitions
│   ├── step_definitions/       # Contains JavaScript files with step definitions
│   │   ├── shopSteps.js       # Step definitions for shop.feature
│   ├── shop.feature           # Gherkin feature file for shop tests
│
├── node_modules/               # Dependencies installed via npm
├── .gitignore                  # Git ignore file to exclude unnecessary files
├── cucumber.js                 # Configuration file for Cucumber.js
├── generateReport.js           # Script to generate custom reports
├── index.html                
├── index.js                    
├── package-lock.json           # Lock file for npm dependencies
├── package.json                # Project configuration file for npm
├── playwright.config.js        # Configuration file for Playwright tests
├── saveReport.js               # Script to save test reports to MongoDB    
├── README.md                   # Project documentation
```

# 🚀 How to Run Tests


## 2️⃣ Install Dependencies

```bash
npm install
```

## 3️⃣ Start the Local Server

```bash
npm start
```

## 4️⃣ Run Tests

```bash
npm test
```


### 🔍 Fetch All Reports

```http
GET /reports
```

This endpoint retrieves all stored reports.

### 🆕 Fetch the Latest Report

```http
GET /latest-report
```

This endpoint retrieves the most recently uploaded test report.

---

This project now supports MongoDB for enhanced test report management, making it easier to store and retrieve test results programmatically! 🎯

