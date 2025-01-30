const { test, expect } = require('@playwright/test');

test.describe('Login Page Tests', () => {

  test('Valid login should redirect to dashboard', async ({ page }) => {
    await page.goto('http://localhost:3000/');
    await page.fill('#username', 'testuser');
    await page.fill('#password', 'password123');
    await page.click('button[type="submit"]');
    
    await page.waitForTimeout(1000);
    await expect(page).toHaveURL(`http://localhost:3000/dashboard`);
  });

  test('Invalid login should show error message', async ({ page }) => {
    await page.goto('http://localhost:3000/');
    await page.fill('#username', 'wronguser');
    await page.fill('#password', 'wrongpassword');
    await page.click('button[type="submit"]');
    
    // Check for error message
    const errorMessage = page.locator('text=Invalid credentials');
    await expect(errorMessage).toBeVisible();
  });
});
