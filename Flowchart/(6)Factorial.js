let n = parseInt(prompt("Enter n : "));
let fact = 1;
let i = 1;

if (n >= 0) {
  while (i <= n) {
    fact *= i;
    i++;
  }

  console.log(fact);
} else {
  console.log("n must be > or = to 0");
}