
// Print Given n Numbr of Series Ex : 1, -2, 3, -4, 5, -6, n...
let n = parseInt(prompt("Enter n : "));
let i = 1

if (n >= 1) {
    
 while(i <= n) {
  if(i % 2 === 0) {
    console.log(i * -1);
  }else
  {
  console.log(i);
  }
  i++
 }
 
}
else{
    console.log("n must be > or = to 1 ");
}