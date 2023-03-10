"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.countBits = void 0;
// function that turns the n into its binary, and counts the
// number of 1s in the binary representation
function countBits(n) {
    // convert n to binary
    var binary = n.toString(2);
    // count the number of 1s in the binary
    var count = 0;
    for (var i = 0; i < binary.length; i++) {
        if (binary[i] === "1") {
            count++;
        }
    }
    return count;
}
exports.countBits = countBits;
// console logs
console.log(countBits(0)); // 0
console.log(countBits(4)); // 1
console.log(countBits(1234)); // 5
