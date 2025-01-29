Feature: Login Form

  As a user
  I want to log in to the application
  So that I can access my account and use the application's features

  Background:
    Given I am on the login page

  Scenario: Successful login with valid credentials
    When I enter "user@example.com" in the email field
    And I enter "password123" in the password field
    And I click the "Login" button
    Then I should be redirected to the dashboard
    And I should see a welcome message

  Scenario: Login fails with invalid email
    When I enter "invalidemail" in the email field
    And I enter "password123" in the password field
    And I click the "Login" button
    Then I should see an error message "Please enter a valid email address"

  Scenario: Login fails with incorrect password
    When I enter "user@example.com" in the email field
    And I enter "wrongpassword" in the password field
    And I click the "Login" button
    Then I should see an error message "Incorrect password"

  Scenario: Login fails with empty email field
    When I leave the email field empty
    And I enter "password123" in the password field
    And I click the "Login" button
    Then I should see an error message "Email is required"

  Scenario: Login fails with empty password field
    When I enter "user@example.com" in the email field
    And I leave the password field empty
    And I click the "Login" button
    Then I should see an error message "Password is required"

  Scenario: Login fails with both fields empty
    When I leave the email field empty
    And I leave the password field empty
    And I click the "Login" button
    Then I should see an error message "Email and password are required"