function fizzBuzz() {
    var results = [];
    var isFizz = function (i) { return i % 3 === 0; };
    var isBuzz = function (i) { return i % 5 === 0; };
    var isFizzBuzz = function (i) { return isFizz(i) && isBuzz(i); };
    for (var i = 1; i < 101; i++) {
        if (isFizzBuzz(i)) {
            results.push('FizzBuzz');
        }
        else if (isFizz(i)) {
            results.push('Fizz');
        }
        else if (isBuzz(i)) {
            results.push('Buzz');
        }
        else {
            results.push("" + i);
        }
    }
    return results;
}
fizzBuzz().map(function (a) { return console.log(a); });
