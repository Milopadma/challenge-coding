// function that retunrs the fibonacci number at the nth position
export function nthFibo(n: number): number {
  let nth = 1;

  // the fibonacci sequence is
  // 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, etc...
  let lastTwo = [0, 1];

  while (true) {
    if (nth === n) {
      return lastTwo[0];
    } else {
      nth++;

      lastTwo = [lastTwo[1], lastTwo[0] + lastTwo[1]];
    }
  }
}

// console log tests
console.log(nthFibo(1)); // 0
console.log(nthFibo(2)); // 1
console.log(nthFibo(3)); // 1
console.log(nthFibo(4)); // 2
console.log(nthFibo(5)); // 3
