const http = require('http');
const fs = require('fs');
const path = require('path');
const url = require('url');
const querystring = require('querystring');

// Dummy credentials (username: testuser, password: password123)
const validCredentials = {
    username: 'testuser',
    password: 'password123',
};

const server = http.createServer((req, res) => {
    const parsedUrl = url.parse(req.url);
    const pathname = parsedUrl.pathname;

    // Serve login page
    if (pathname === '/') {
        if (req.method === 'GET') {
            fs.readFile(path.join(__dirname, 'index.html'), 'utf8', (err, data) => {
                if (err) {
                    res.writeHead(500, { 'Content-Type': 'text/plain' });
                    return res.end('Error reading the HTML file');
                }
                res.writeHead(200, { 'Content-Type': 'text/html' });
                res.end(data);
            });
        }
    }
    // Handle login form submission
    else if (pathname === '/login' && req.method === 'POST') {
        let body = '';
        req.on('data', chunk => {
            body += chunk;
        });

        req.on('end', () => {
            const parsedBody = querystring.parse(body);
            const { username, password } = parsedBody;

            // Check if credentials are valid
            if (username === validCredentials.username && password === validCredentials.password) {
                res.writeHead(302, { 'Location': '/dashboard' });
                res.end();
            } else {
                res.writeHead(401, { 'Content-Type': 'text/html' });
                res.end('<h1>Invalid credentials. Please try again.</h1><a href="/">Go back</a>');
            }
        });
    }
    // Serve dashboard page after successful login
    else if (pathname === '/dashboard') {
        res.writeHead(200, { 'Content-Type': 'text/html' });
        res.end('<h1>Welcome to your Dashboard!</h1><a href="/">Log out</a>');
    }
    // Handle 404 errors
    else {
        res.writeHead(404, { 'Content-Type': 'text/plain' });
        res.end('Page not found');
    }
});

server.listen(3000, () => {
    console.log('Server running at http://localhost:3000/');
});
