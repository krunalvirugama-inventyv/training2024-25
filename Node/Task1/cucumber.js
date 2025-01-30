module.exports = {
  default: {
    require: ["features/step_definitions/*.js"],
    requireModule: ["@babel/register"],
    format: ["progress-bar", "json:cucumber_report/cucumber_report.json"],
    publishQuiet: true
  }
};