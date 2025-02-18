# Multi-threaded User Management Program

## Overview

This program demonstrates a multi-threaded user management system in Rust using concurrency primitives such as `Arc` and `RwLock`. It manages a list of `User` objects, each containing an `id` and a `timestamp`. Multiple threads interact with this shared list concurrently to perform various operations such as creating users, printing the list, cleaning up old entries, and stopping the program when a condition is met.

## Functionality

The program consists of five separate threads, each with a specific responsibility:

### Thread 1 - User Creation

This thread continuously creates a new `User` object every 500 milliseconds. Each user is assigned a unique ID and a timestamp representing the current system time in seconds since the UNIX epoch. The new user is added to the shared vector.

### Thread 2 - User Count Display

This thread prints the current length of the user vector every 1 second. It provides insight into the number of users currently stored in the vector.

### Thread 3 - Data Cleanup

This thread removes users from the vector whose timestamps are older than 2 seconds compared to the current system time. It performs this cleanup operation every 3 seconds.

### Thread 4 - Full User List Display

This thread prints the entire contents of the user vector every 5 seconds. It allows for visualization of the user data at regular intervals.

### Thread 5 - Program Stop Condition

This thread monitors the length of the user vector. If the vector exceeds 7 users, this thread sets a shared `is_stop` flag to `true`, causing all other threads to terminate and stopping the program.

## Synchronization and Thread Safety

The program uses `Arc<RwLock<Vec<User>>>` to safely share the user vector across multiple threads.

- `Arc` (Atomic Reference Counting) is used to enable multiple threads to own a reference to the same data.
- `RwLock` (Read-Write Lock) allows multiple threads to read the data concurrently, while ensuring exclusive access for writing.

Each thread clones the `Arc` reference to gain access to the shared user vector and the `is_stop` flag. When accessing the vector, threads acquire a read or write lock depending on whether they are reading or modifying the data.

## Program Termination

The `is_stop` flag is monitored by each thread in their respective loops. When Thread 5 detects that the user vector has more than 7 entries, it sets `is_stop` to `true`. This causes all threads to exit their loops and the program to terminate gracefully.

## Expected Output

The program outputs the following types of messages:

- Creation of new users with their ID and timestamp.
- Current length of the user vector.
- Cleanup activity indicating that old users have been removed.
- Full contents of the user vector.
- A message indicating that the program has been stopped when the user vector exceeds 7 entries.

## Key Points

- Concurrency is achieved using `Arc` and `RwLock`.
- Threads perform distinct tasks such as creating, displaying, cleaning, and monitoring users.
- The program terminates automatically when the user vector exceeds a specific size.

## Example Use Cases

- Simulating concurrent data processing.
- Learning and practicing Rust concurrency primitives.
- Building real-time data monitoring systems with multiple operations happening in parallel.

This program serves as a practical example of multi-threading and synchronization in Rust, providing a foundation for more complex concurrent systems.

