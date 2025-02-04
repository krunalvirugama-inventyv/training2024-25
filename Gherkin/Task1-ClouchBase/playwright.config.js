module.exports = {
  use: {
    browserName: 'chromium',
    headless: false,
    viewport: { width: 1280, height: 720 },
    slowMo: 50,
    video: 'on-first-retry',
    executablePath: '/path/to/custom/chromium',
  },
  retries: 2,
  timeout: 30000,
  reporter: [['html', { open: 'never' }], ['json', { outputFile: 'results.json' }]],
  workers: 4,
  // projects: [
  //   {
  //     name: 'Chromium Tests',
  //     testDir: './tests/chromium',
  //     use: { browserName: 'chromium' },
  //   },
  //   {
  //     name: 'Firefox Tests',
  //     testDir: './tests/firefox',
  //     use: { browserName: 'firefox' },
  //   },
  // ],
};
