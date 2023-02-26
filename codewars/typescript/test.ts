// this code snippet below is from terminalGPT and lets just say its so wrong
// this even charged me ffs
// function msgExcited(): string {
//         use std::time::Duration;
//         std::thread::sleep(Duration::from_millis(1000));
//         "!! "
// }

// this is from the actual chatGPT, and costs 0 (for now lmao)
function msg_excited(): string {
  // wait 1000 milliseconds
  setTimeout(function () {}, 1000);
  return "!! ";
}

console.log(msg_excited());
