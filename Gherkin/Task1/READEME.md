# Task1: Test Automation Project 🚀

![Testing](https://img.shields.io/badge/Tests-Playwright%20%26%20Cucumber-blue)


## 📋 What is Task1?
A test automation project combining **Playwright** and **Cucumber.js** for end-to-end testing, focused on login functionality. Includes dual reporting systems and custom report generation.

A structured test automation project that demonstrates:
- **Playwright**: For browser automation and UI testing.
- **Cucumber.js**: For BDD (Behavior-Driven Development) tests using Gherkin syntax.
- **Dual Reporting**: HTML/JSON reports for both frameworks.
- **Custom Reports**: Script to merge/format results via `generateReport.js`.

## 🛠️ Prerequisites
- Node.js v16+ ([Download](https://nodejs.org/))
- npm (comes with Node.js)
- Modern browser (Chromium, Firefox, or WebKit)


## 📂 Project Structure
```
TASK1/
├── cucumber_report/            # Cucumber reports generated after running tests
│   ├── cucumber_report.html    # HTML report for Cucumber tests
│   ├── cucumber_report.json    # JSON report for Cucumber tests
│
├── features/                   # Cucumber test features and step definitions
│   ├── step_definitions/       # Contains JavaScript files with step definitions
│   │   ├── loginSteps.js       # Step definitions for login.feature
│   ├── login.feature           # Gherkin feature file for login tests
│
├── node_modules/               # Dependencies installed via npm
│
├── playwright-report/          # Playwright reports generated after running tests
│   ├── index.html              # Playwright HTML report
│   ├── playwright-report.json  # JSON report for Playwright tests
│
├── test/                       # Playwright test scripts
│   ├── login.test.js           # Playwright test script for login functionality
│
├── test-results/               # Folder to store test result artifacts
│
├── .gitignore                  # Git ignore file to exclude unnecessary files
├── cucumber.js                 # Configuration file for Cucumber.js
├── generateReport.js           # Script to generate custom reports
├── index.html                  
├── index.js                   
├── package-lock.json           # Lock file for npm dependencies
├── package.json                # Project configuration file for npm
├── playwright.config.js        # Configuration file for Playwright tests

```


# 🚀 How to Run Tests

## First Start a Local Server
```bash
npm start
```

## Run Cucumber Tests + Generate Cucumber Report
```bash
npm test
```

## Run Playwright Tests + Generate Playwright Report
```bash
npm run reportp
```


