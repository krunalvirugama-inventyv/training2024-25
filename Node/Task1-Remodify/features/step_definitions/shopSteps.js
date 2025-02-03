const {
    Given,
    When,
    Then,
    AfterAll
} = require('@cucumber/cucumber');
const {
    chromium
} = require('playwright');

const {
    expect
} = require('@playwright/test');
const axios = require('axios');
let browser;
let page;

Given('the user is on the Flipkart website', {
    timeout: 15000
}, async () => {
    browser = await chromium.launch({
        headless: true
    }); // Set headless to false for debugging

    page = await browser.newPage();

    await page.goto('https://www.flipkart.com/', {
        waitUntil: 'networkidle'
    });
});
When('the user types {string} in the search bar and clicks "Search"', {
    timeout: 15000
}, async (search_word) => {

    await page.getByRole('textbox', {
        name: 'Search for Products, Brands'
    }).click();
    await page.getByRole('textbox', {
        name: 'Search for Products, Brands'
    }).fill(search_word);
    await page.getByRole('button', {
        name: 'Search for Products, Brands'
    }).click();
    await page.waitForLoadState('networkidle');
});


Then("the search results should show relevant products", async () => {
    await page.waitForSelector('.KzDlHZ', {
        timeout: 10000
    });
});

Then("the first product's name should be similar to the search word {string}", async (search_word) => {

    const firstProductName = await page.locator('.KzDlHZ').first().textContent();

    // console.log(`prdocut = ${search_word} result = ${firstProductName}`);

    const productNameMatch = firstProductName.trim().toLowerCase().includes(search_word.toLowerCase());

    // Assert that the search word is found in the first product name
    expect(productNameMatch).toBe(true);

    await browser.close();
});


// After all scenarios, close the browser
AfterAll(async () => {
  

    const apiEndpoint = 'http://localhost:3000/save-report';

    await axios.post(apiEndpoint)
        .then(response => {
            console.log('Data sent successfully');
        })
        .catch(error => {
            console.error('Error sending data:', error.message);
        });

    console.log('Browser closed after all scenarios');
});