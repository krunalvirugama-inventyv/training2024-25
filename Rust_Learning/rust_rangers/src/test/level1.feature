Feature: Level 1 - Forest of Variables
    
    As a Forest caretaker
    I want to manage a number of teak trees in the Forest
    so than I can ensure the forest thrives and maintains its balance
    
    Scenario: Planting a new teak tree
        Given the forest starts with 500 teak trees
        And the teaktree varibale is mutable
        Where I plant "50" trees planted by "Krunal"
        Then the total number of teak trees should be "550"
    
    Scenario: Cut trees with permission
        Given the forest with 550 teak trees
        And the teaktree varibale is mutable
        And I Have permission to cut tree
        When I cut "30" trees
        Then the total number of teak trees should be "520"
