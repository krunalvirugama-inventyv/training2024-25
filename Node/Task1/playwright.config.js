module.exports = {

  reporter: [
    ['html', {
      outputFolder: 'playwright-report',
      open: 'always'
    }],
    ['json', {
      outputFile: 'playwright-report/playwright-report.json'
    }],
  ],

};