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
    }
  }
  //return the sum of the minimum values
  return min.reduce((a, b) => a + b, 0);
}
