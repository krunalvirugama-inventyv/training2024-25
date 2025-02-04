Feature: Search for a Product

  Scenario Outline: User verifies product names in the search results
    Given the user is on the Flipkart website
    When the user types "<search_word>" in the search bar and clicks "Search"
    Then the search results should show relevant products
    And the first product's name should be similar to the search word "<search_word>"

    Examples:
      | search_word |
      | vivo t3x    |
      | samsung s10 |
     
  
