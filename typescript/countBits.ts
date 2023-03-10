// function that turns the n into its binary, and counts the
// number of 1s in the binary representation
export function countBits(n: number): number {
  // convert n to binary
  let binary = n.toString(2);

  // count the number of 1s in the binary
  let count = 0;
  for (let i = 0; i < binary.length; i++) {
    if (binary[i] === "1") {
      count++;
    }
  }

  return count;
}

// console logs
console.log(countBits(0)); // 0
console.log(countBits(4)); // 1
console.log(countBits(1234)); // 5
