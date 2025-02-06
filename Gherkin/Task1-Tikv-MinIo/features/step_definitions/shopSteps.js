const {
    Given,
    When,
    Then,
    After,
    AfterAll,
    setDefaultTimeout,
    Before
} = require('@cucumber/cucumber');
const {
    chromium
} = require('playwright');

const path = require('path');

const Minio = require('minio');
const fs = require('fs'); // Ensure fs is required
const {
    uploadToMinio
} = require('../../utils/minioClient');

const {
    expect
} = require('@playwright/test');

let browser;
let page;
let context

const screenshotDir = './screenshot';

if (!fs.existsSync(screenshotDir)) {
    console.log('Creating screenshot directory');
    fs.mkdirSync(screenshotDir, {
        recursive: true
    }); // Ensure the directory is created
}


// Set default timeout to 30 seconds
setDefaultTimeout(30 * 1000);

Before(async function () {
    // Define video storage folder
    browser = await chromium.launch({
        headless: true
    }); // Launch browser
    context = await browser.newContext({
        recordVideo: {
            dir: "./videos",
            size: {
                width: 1280,
                height: 720
            }
        }, // Enable video recording
    });

    page = await context.newPage(); // Create a new page
});



Given('the user is on the Flipkart website', {
    timeout: 15000
}, async () => {

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


});


After(async function (scenario) {

    const videoPath = await page.video().path();
    const videoFileName = path.basename(videoPath);

    try {

        await uploadToMinio(
            'reports',
            `videos/${videoFileName}`,
            videoPath
        );

        console.log("Video uploaded to MinIO");
    } catch (error) {
        console.error("Error uploading video to MinIO:", error);
    }

    fs.unlinkSync(videoPath);

    if (scenario.result.status === 'FAILED') {

        const screenshotPath = path.join(
            screenshotDir,
            `${scenario.pickle.name.replace(/\s+/g, '_')}_${Date.now()}.png`
        );

        await page.screenshot({
            path: screenshotPath
        });

        console.log(`Screenshot saved at: ${screenshotPath}`);

        try {
            await uploadToMinio(
                'reports',
                `screenshots/${path.basename(screenshotPath)}`,
                screenshotPath
            );

            console.log('Screenshot uploaded to MinIO');
        } catch (error) {
            console.error('Error uploading screenshot to MinIO:', error);
        }

        // Clean up the local file after uploading
        fs.unlinkSync(screenshotPath);
    }


    await context.close();
    await browser.close();
});