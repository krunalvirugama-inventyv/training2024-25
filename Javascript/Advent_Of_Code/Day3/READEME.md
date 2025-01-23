# Advent of Code 2025 - Day 3: Mull It Over

### Task 1: Scan and Multiply
In this task, you're given corrupted memory containing various instructions. The program is meant to multiply numbers using `mul(X,Y)` instructions, but the memory has been corrupted with invalid characters. Your job is to scan the corrupted memory for valid `mul(X,Y)` instructions and compute the sum of the results.

A valid `mul(X,Y)` instruction multiplies two numbers and produces a result. Invalid characters and sequences should be ignored. For example, `mul(2,4)` is a valid instruction, while `mul(4*` is not.

Your goal is to extract the valid `mul(X,Y)` instructions, multiply the numbers, and sum the results.

### Task 2: Handle Conditional Instructions
In this part of the puzzle, you must account for two new instructions: `do()` and `don't()`. These instructions control whether future `mul(X,Y)` instructions are enabled or disabled.

- The `do()` instruction enables future `mul` instructions.
- The `don't()` instruction disables future `mul` instructions.
- At the start of the program, `mul` instructions are enabled by default.

Your task is to handle these conditional instructions and compute the sum of the results for only the enabled `mul(X,Y)` instructions.

For example:
- After `don't()` is encountered, subsequent `mul(X,Y)` instructions are ignored until `do()` is encountered again.
  
After processing the entire input with these new conditions, calculate the sum of all the enabled `mul` results.

---
