// Print Given n Numbr of Series Ex : 1!, 3!, 5!, 7!, 9!, ...n

let n = parseInt(prompt("Enter n : "));
let i = 3;
let j = 1;
let fact = 1;

if (n > 0) {
     while (j <= n) {
        console.log(fact);
        fact *= i * (i - 1);
        i += 2;
        j +=1;
    }
} else {
   console.log("n must be > 0");
}