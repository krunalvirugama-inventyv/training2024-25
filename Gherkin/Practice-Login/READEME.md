# (Task1) Login Form Test Scenarios with Gherkin


## Folder Structure

```
Task1/
├── Task1.feature
│── Task1WithBg.feature
│── Task1WithTable.md
└── README.md
```

### Task Files Description
1. **`Task1.feature`**  
   Basic test scenarios for a login form, including valid/invalid credentials and empty fields.

2. **`Task1WithBg.feature`**  
   Uses the `Background` keyword to define common preconditions (e.g., navigating to the login page) reused across scenarios.

3. **`Task1WithTable.feature`**  
   Uses **data tables** to test multiple input combinations (email/password) in a single scenario outline.
