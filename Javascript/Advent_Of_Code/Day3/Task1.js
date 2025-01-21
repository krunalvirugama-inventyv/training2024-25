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

    const regex = /mul\(\d{1,3},\d{1,3}\)/g;

    let foundMulArray = data.toString().match(regex);

    const sumOfMulStr = mulAndSum(foundMulArray);

    console.log("Sum = ", sumOfMulStr);
});
