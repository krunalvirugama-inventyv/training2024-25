const { Given, When, Then } = require('@cucumber/cucumber');
const { chromium } = require('playwright');

let browser;
let page;

Given('I am on the login page', async () => {
    browser = await chromium.launch({ headless: true });  // Set headless to false for debugging
    page = await browser.newPage();
    await page.goto('http://localhost:3000/');
});

When('I enter valid credentials', async () => {
    await page.fill('#username', 'testuser');
    await page.fill('#password', 'password123');
    await page.click('button[type="submit"]');
});

When('I enter invalid credentials', async () => {
    await page.fill('#username', 'wronguser');
    await page.fill('#password', 'wrongpassword');
    await page.click('button[type="submit"]');
});

Then('I should be redirected to the dashboard', async () => {
    // Delay to ensure navigation completes
    await page.waitForTimeout(1000);  // Wait for 1 second
    // await page.waitForNavigation();
    const url = page.url();
    if (url === 'http://localhost:3000/dashboard') {
        console.log('Test Passed: User is redirected to dashboard');
    } else {
        console.log('Test Failed: User is not redirected');
    }
    await browser.close();
});



Then('I should see an error message', async () => {
    const errorMessage = await page.locator('text=Invalid credentials').isVisible();
    if (errorMessage) {
        console.log('Test Passed: Error message displayed');
    } else {
        console.log('Test Failed: Error message not displayed');
    }
    await browser.close();
});
