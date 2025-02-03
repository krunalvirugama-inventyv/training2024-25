module.exports = {
  reporter: [
    // testDir: './test', // Path to Playwright test files (optional)
    ['html', {
      outputFolder: 'playwright-report',
      open: 'always'
    }],
    ['json', {
      outputFile: 'playwright-report/playwright-report.json'
    }],
  ],
};