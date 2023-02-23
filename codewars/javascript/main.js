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

// function to return the count of the number of each letter in a string
const orderedCount = function (text) {
  // store the count of each letter in an object outside the loop
  let letterCount = {};
  // loop through the string
  for (let i = 0; i < text.length; i++) {
    // if the letter is not in the object, add it with a count of 1
    if (!letterCount[text[i]]) {
      letterCount[text[i]] = 1;
    } else {
      // if the letter is in the object, increment the count by 1
      letterCount[text[i]]++;
    }
  }
  // return the object as an array of arrays
  return Object.entries(letterCount);
};

console.log(orderedCount("abracadabra"));
