const fs = require('fs');

function mulAndSum(array) {
    let sum = 0;
    for (element of array) {
        const match = element.match(/(\d+)/g);
        ans = parseInt(match[0]) * parseInt(match[1]);
        sum += ans;
    }
    return sum;
}

fs.readFile('String.txt', (err, data) => {
    if (err) throw err;
   data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)don't()+mul(32,64](mul(11,8)undo()?mul(8,5))";
   console.log(data);
   
    const regx1 = /don't\(\).*?do\(\)/g;
    const regex2 = /mul\(\d{1,3},\d{1,3}\)/g;

    const inputString = data.toString().replace(regx1, '');

    console.log(inputString);
    
    let foundMulArray = inputString.toString().match(regex2);

    const sumOfMulStr = mulAndSum(foundMulArray);

    console.log("Sum = ", sumOfMulStr);
});


