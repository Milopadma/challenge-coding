// sum of minimums
function sumOfMinimums(arr) {
  let min = [];
  for (let i = 0; i < arr.length; i++) {
    for (let j = 0; j < arr[i].length; j++) {
      // find the minimum value in each array
      min.push(Math.min(...arr[i]));
    }
  }
  //return the sum of the minimum values
  return min.reduce((a, b) => a + b, 0);
}

function sumOfMinimum(arr) {
  let min = [];
  for (let i = 0; i < arr.length; i++) {
    min.push(Math.min(...arr[i]));
  }
  return min.reduce((a, b) => a + b, 0);
}
