// Print Given n Numbr of Series Ex : 1,1,2,1,1,2,3,2,1,1,2,3,4,3,2,1 ...n

let n = parseInt(prompt("Enter n : "));
let i = 1;
let j = 1;

if (n > 0) {
    console.log(j);
     
     while (i < n) {
       
       let line = "";
       j =1;
       
       while(j<=i)
       {   
           line += j + " ";
           j++;
       }
       
       while(j>=1){
           line += j + " ";
           j--;
       }
       
       console.log(line.trim());
       i++;
    }
} else {
   console.log("n must be > 0");
}




