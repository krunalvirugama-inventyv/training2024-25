# Advent of Code 2024 - Day 1: Historian Hysteria

## Overview

The Chief Historian is missing, and the Senior Historians need your help to reconcile their lists of historically significant location IDs. Your mission is to assist in analyzing these lists to ensure all important locations are checked, saving Christmas in the process!

This challenge consists of two tasks:

- **Task 1:** Calculate the total distance between the two lists of location IDs.
- **Task 2:** Calculate a similarity score based on how frequently numbers from the left list appear in the right list.

---

## Day 1: Task 1 - Total Distance Between Lists

### Problem Statement

The Senior Historians split into two groups to compile a list of location IDs. However, their lists are not identical. To reconcile the lists, the total distance between the numbers in both lists needs to be calculated.

1. Pair up the smallest number in the left list with the smallest number in the right list, then the second smallest with the second smallest, and so on.
2. For each pair, calculate the absolute difference.
3. Sum all the absolute differences to determine the total distance


## Day 1: Task 2 - Similarity Score

### Problem Statement

#### The historians suspect that a significant portion of the numbers from both lists are similar. Calculate a similarity score by:


1. Counting how many times each number from the left list appears in the right list.
2. Multiplying each number in the left list by the count of its occurrences in the right list.
3. Summing all these values to calculate the total similarity score.
