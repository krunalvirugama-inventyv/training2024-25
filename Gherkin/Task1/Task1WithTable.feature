Feature: Login Form

  As a user
  I want to log in to the application
  So that I can access my account and use the application's features

  Background:
    Given I am on the login page

  Scenario Outline: Login with different email and password combinations
    When I enter <email> in the email field
    And I enter <password> in the password field
    And I click the "Login" button
    Then I should see <message>

    Examples:
      | email               | password     | message                              |
      | user@example.com    | password123  | Welcome to the dashboard             |
      | invalidemail        | password123  | Please enter a valid email address   |
      | user@example.com    | wrongpassword| Incorrect password                   |
      |                     | password123  | Email is required                    |
      | user@example.com    |              | Password is required                 |
      |                     |              | Email and password are required      |