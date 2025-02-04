const {
    Given,
    When,
    Then,
} = require('@cucumber/cucumber');
const {
    chromium
} = require('playwright');

const {
    expect
} = require('@playwright/test');

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

