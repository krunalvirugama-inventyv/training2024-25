const report = require("cucumber-html-reporter");
const path = require("path");

const options = {
  theme: "bootstrap", // Available themes: 'bootstrap', 'hierarchy', 'foundation', 'simple'
  jsonFile: path.join(__dirname, "cucumber_report/cucumber-report.json"),
  output: path.join(__dirname, "cucumber_report/custom-cucumber-report.html"),
  reportSuiteAsScenarios: true,
  launchReport: true, // Opens the report in the browser automatically
  metadata: {
    "App Version": "1.0.0",
    "Test Environment": "STAGING",
    "Browser": "Chrome",
    "Platform": "Windows 10",
  },
};

report.generate(options);
console.log("Custom Cucumber Report Generated Successfully!");
