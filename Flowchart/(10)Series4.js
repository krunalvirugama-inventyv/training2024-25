// Print Given n Numbr of Series Ex : 1,1,2,1,2,3,1,2,3,4, ...n

let n = parseInt(prompt("Enter n : "));
let i = 1;
let j = 1;

if (n > 0) {
     while (i <= n) {
       j =1 ;
       while(j<=i)
       {
           console.log(j);
           j++;
       }
       i++;
        
    }
} else {
   console.log("n must be > 0");
}
