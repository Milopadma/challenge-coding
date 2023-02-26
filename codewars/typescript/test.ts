// this code snippet below is from terminalGPT and lets just say its so wrong
// this even charged me ffs
// function msgExcited(): string {
//         use std::time::Duration;
//         std::thread::sleep(Duration::from_millis(1000));
//         "!! "
// }

// this is from copilot
function msg_excited(): string {
  // wait 1000 milliseconds
  setTimeout(function () {}, 1000);
  return "!! ";
}

// this is from code-davinci-002
function quickSort_davinci_002(list: Array<number>): Array<number> {
  if ((list.length = 1)) {
    return list;
  }
  let pivot: number = list.shift();
  let lesser: Array<number> = list.filter((x) => x < pivot);
  let greater: Array<number> = list.filter((x) => x >= pivot);

  return quickSort_davinci_002(lesser).concat(
    pivot,
    quickSort_davinci_002(greater)
  );
}

// this is from copilot straight
function quickSort(list: Array<number>): Array<number> {
  if (list.length <= 1) {
    return list;
  }

  const pivot = list[0];
  // this code below works but quicksort is great ,
  // because it does not need memory allocation
  //   const left = [];
  //   const right = [];

  //   for (let i = 1; i < list.length; i++) {
  //     if (list[i] < pivot) {
  //       left.push(list[i]);
  //     } else {
  //       right.push(list[i]);
  //     }
  //   }

  // this is proper implementation for quicksort
  // if x is less than pivot, then add it to left
  const left = list.slice(1).filter((x) => x < pivot);
  const right = list.slice(1).filter((x) => x >= pivot);

  return [...quickSort(left), pivot, ...quickSort(right)];
}

console.log(quickSort([5, 7, 2, 6, 1, 2]));

console.log(msg_excited());
