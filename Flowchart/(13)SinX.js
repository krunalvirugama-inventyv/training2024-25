const PI = 3.14159265;

function calculateSine(x, n) {
    // Convert x from degrees to radians
    x = (x * PI) / 180;

    let temp = x; // First term of the series
    let sum = x; // Initialize sum with the first term

    for (let i = 1; i < n; i++) {
        temp = temp * (-1) * x * x / ((2 * i) * (2 * i + 1));
        sum += temp;
    }
    return sum;
}

// Input
const x = parseFloat(prompt("Enter the value of x in degrees:"));
const n = parseInt(prompt("Enter the number of terms (n):"), 10);

// Calculation
const sineValue = calculateSine(x, n);

// Output
console.log(`Sine of x = ${sineValue.toFixed(6)}`);
