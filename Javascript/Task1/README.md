# Login and Registration with JavaScript, HTML, and Bootstrap

## Overview
This project is a simple login and registration system that allows users to register, log in, and be greeted with a welcome message. The application uses JavaScript for handling form submissions, storing data in the browser's `localStorage`, and managing user authentication through cookies. Bootstrap is used to style the page and ensure responsiveness.



## Folder Structure

```
Task1/
├── index.html
├── welcome.html
├── style.css
├── Logo.svg
├── README.md
```

- `index.html`: The login and registration page.
- `welcome.html`: The welcome page that shows the logged-in user's name and a session countdown.
- `style.css`: Custom CSS for additional styling (optional, can be replaced with Bootstrap's styles).
- `Logo.svg`: Logo image for the profile section.



## Technologies Used
- **HTML5**: Structure and content of the page.
- **CSS3**: Styling for the page layout and design (via Bootstrap).
- **JavaScript**: Handles form validation, user registration, login, and session management using cookies and `localStorage`.
- **Bootstrap 5**: For responsive design and UI components.

## Features
- **User Registration**: Users can register by providing a username, email, and password. The data is stored in the browser's `localStorage`.
- **Login System**: Users can log in with their username and password. If credentials are correct, a session cookie is set, and the user is redirected to a welcome page.
- **Session Management**: The session expires after 40 seconds, displaying a countdown and logging out automatically when the timer reaches 0. Users can also log out manually by clicking the logout button.
- **Responsive Design**: The layout is mobile-friendly and adjusts for different screen sizes using Bootstrap.
- **User Feedback**: Alerts are shown for successful registration, login, and invalid attempts.

## How It Works
### Registration
1. The user enters a username, email, and password.
2. The data is validated, and if the username or email already exists, an alert is shown.
3. If registration is successful, the user is added to the `localStorage` and a success message is shown.

### Login
1. The user enters their username and password.
2. The credentials are validated against the stored users in `localStorage`.
3. If correct, the user is logged in, and a session cookie is set.
4. The user is redirected to a welcome page with a countdown timer.

### Session Timeout
1. A countdown starts when the user is logged in.
2. If the countdown reaches 0, the user is automatically logged out.
3. The user can also manually log out by clicking the logout button.

## Getting Started
1. Clone the repository or download the files.
2. Open `index.html` in your browser to view the login and registration page.
3. Enter a username and password to register or log in.
4. Once logged in, a session countdown will start, and you can manually log out.
