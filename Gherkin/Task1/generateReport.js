const reporter = require('cucumber-html-reporter');

const options = {
  theme: 'bootstrap',
  jsonFile: './cucumber_report/cucumber_report.json',
  output: './cucumber_report/cucumber_report.html',
  reportSuiteAsScenarios: true,
  launchReport: true,
};

reporter.generate(options);
