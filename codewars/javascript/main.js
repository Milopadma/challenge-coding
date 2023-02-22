// function that returns the sum of the minimum values in each array
function sumOfMinimum(nestedArray) {
  // create an empty array to store the minimum values
  let min = [];
  // loop through the nested array
  for (let i = 0; i < nestedArray.length; i++) {
    // loop through each array in the nested array
    for (let j = 0; j < nestedArray[i].length; j++) {
      // find the minimum value in each array
      min.push(Math.min(...nestedArray[i]));
      console.log(min);
    }
  }
  //remove duplicates
  min = [...new Set(min)];

  //return the sum of the minimum values
  return min.reduce((a, b) => a + b, 0);
}

console.log(
  sumOfMinimum([
    [1, 2, 3, 4, 5],
    [5, 6, 7, 8, 9],
    [20, 21, 34, 56, 100],
  ])
);
