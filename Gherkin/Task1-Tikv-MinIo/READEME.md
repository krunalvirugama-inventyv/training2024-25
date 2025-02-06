# Task1: Test Automation Project with TiKV and MinIO 🚀

## 📋 What is Task1?

A test automation project combining **Playwright** and **Cucumber.js** for end-to-end testing, now enhanced with **TiKV** and **MinIO** integration for report storage, retrieval, and media management (screenshots and video recordings).

### Key Features:

- **Playwright**: Browser automation and UI testing.
- **Cucumber.js**: BDD tests using Gherkin syntax.
- **Custom Reports**: Script to merge/format results via `generateReport.js`.
- **TiKV Integration**: Store and retrieve reports from the TiKV distributed database.
- **MinIO Integration**: Upload failure screenshots and video recordings to MinIO.
- **Failure Handling**: Capture screenshots and video recordings on test failure.

## 🛠️ Prerequisites

- Node.js v16+ ([Download](https://nodejs.org/))
- npm (comes with Node.js)
- TiKV ([Download](https://tikv.org/)) or TiKV Cloud
- MinIO ([Download](https://min.io/)) or MinIO Cloud
- Modern browser (Chromium, Firefox, or WebKit)

## 📂 Project Structure


```
TASK1/
├── cucumber_report/            # Cucumber reports generated after running tests
│   ├── cucumber_report.html    # HTML report for Cucumber tests
│   ├── cucumber_report.json    # JSON report for Cucumber tests
│
├── dbConfig/                   # Database configuration folder
│   ├── db.js                   # Couchbase connection setup
│
├── features/                   # Cucumber test features and step definitions
│   ├── step_definitions/       # Contains JavaScript files with step definitions
│   │   ├── shopSteps.js       # Step definitions for shop.feature
│   ├── shop.feature           # Gherkin feature file for shop tests
│
├── node_modules/               # Dependencies installed via npm
├── test-results/               # Folder to store test result artifacts
├── .gitignore                  # Git ignore file to exclude unnecessary files
├── cucumber.js                 # Configuration file for Cucumber.js
├── generateReport.js           # Script to generate custom reports
├── index.html                  
├── index.js                    
├── package-lock.json           # Lock file for npm dependencies
├── package.json                # Project configuration file for npm
├── playwright.config.js        # Configuration file for Playwright tests
├── saveReport.js               # Script to save test reports to Couchbase    
├── README.md                   # Project documentation
```
## 🚀 How to Run Tests

### 1️⃣ Set Up TiKV & MinIO
Ensure **TiKV** and **MinIO** are running and properly configured.

### 2️⃣ Install Dependencies

```bash
npm install
```

## 3️⃣ Start the Local Server

```bash
npm start
```

## Failure Handling
In the event of a test failure, screenshots and video recordings are captured and uploaded to MinIO. This allows for easier debugging and review of test failures.

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

### 💻 Failure Handling Implementation
When a test fails, the following actions are triggered:

- Screenshot Capture: A screenshot is captured of the browser window during the test failure.
- Video Recording: The video of the failed test execution is recorded and saved.
- MinIO Upload: Both the screenshot and video are uploaded to the MinIO server for storage.
- This ensures that detailed failure reports are available for review and debugging.


---

By integrating TiKV for report management and MinIO for media storage, this project enhances test automation by providing a streamlined approach to report and failure management. 🎯

