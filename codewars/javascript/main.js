// function that returns the sum of the minimum values in each array
function sumOfMinimum(nestedArray) {
  let sum = 0;
  for (let i = 0; i < nestedArray.length; i++) {
    let min = nestedArray[i][0];
    for (let j = 0; j < nestedArray[i].length; j++) {
      if (nestedArray[i][j] < min) {
        min = nestedArray[i][j];
      }
    }
    sum += min;
  }
  return sum;
}

console.log(
  sumOfMinimum([
    [1, 2, 3, 4, 5],
    [5, 6, 7, 8, 9],
    [20, 21, 34, 56, 100],
  ])
);
