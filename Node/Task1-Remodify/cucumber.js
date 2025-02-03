module.exports = {
  default: {
    require: ["features/step_definitions/*.js"],
    //   paths: ['features/*.feature'],     // Load all feature files
    requireModule: ["@babel/register"],
    "format": ["html:./cucumber_report/cucumber-report.html", "json:./cucumber_report/cucumber-report.json"],
    publishQuiet: true,
    // "tags": "@smoke",
    "publish": true,
    "dry-run": false,
    "profile": false,
    // "max-duration": 30000,
    "retry": 2,
    "backtrace": true,
    "snippets": true,
    "parallel": 4
  }
};  

