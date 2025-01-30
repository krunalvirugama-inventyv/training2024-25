# Advent of Code 2025 - Day 2: Red-Nosed Reports

### Task 1: Identifying Safe Reports
In this task, you are asked to analyze a series of reports from the Red-Nosed reactor. Each report consists of a sequence of levels, and you need to determine how many of these reports are "safe."

A report is considered safe if:
1. The levels are either strictly increasing or strictly decreasing.
2. The difference between any two adjacent levels is between 1 and 3.

You need to compute how many reports meet these criteria.

### Task 2: Incorporating the Problem Dampener
In this part, you need to modify your solution to account for the Problem Dampener. This module allows the reactor safety systems to tolerate a single "bad" level in a report that would otherwise be unsafe. If removing one level from an unsafe report makes it safe, that report counts as safe.

You must update your analysis to handle this new rule and calculate how many reports are now considered safe.

---
