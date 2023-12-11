async function fizz_buzz() {
    var count = 0;
    for (let i = 0; i <= 301; i++) {
        if (i % 3 === 0 && i % 5 === 0) {
            count++;
            console.log("FizzBuzz" + " " + count);
        }
        else if (i % 3 === 0) {
            console.log("Fizz");
        }
        else if (i % 5 === 0) {
            console.log("Buzz");
        }
    }
    console.log("Total FizzBuzz: " + count);
}

fizz_buzz();