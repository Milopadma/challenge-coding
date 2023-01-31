export function nthFibo(n: number): number {
  let nth = 0;

  // the fibonacci sequence is
  // 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, etc...
  // to find the nth fibonacci number
  let lastTwo = [0, 1];

  while (true) {
    // the current fibonacci number is the sum of the last two numbers
    let currentFibo = lastTwo[0] + lastTwo[1];

    // check if the current number is a valid fibonacci number
    if (currentFibo === n) {
      console.log("currentFibo: " + currentFibo);
      return nth;
    } else {
      // increase our current nth number
      nth += 1;

      // update the last two integers for the fibonacci sequence
      lastTwo = [lastTwo[1], lastTwo[0] + lastTwo[1]];
    }
  }
}

console.log(nthFibo(1));
console.log(nthFibo(2));
console.log(nthFibo(3));
console.log(nthFibo(4));
console.log(nthFibo(7));
