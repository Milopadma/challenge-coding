export function nthFibo(n: number): number {
  let nth = 0;
  // to find the nth fibonacci number
  while (true) {
    let current = 1;
    let lastTwo = [0, 1];
    if (current == n) {
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
