import * as wasm from "weblets";

var startTime, endTime;

function start() {
    startTime = new Date();
}

function end() {
    endTime = new Date();
    var timeDiff = endTime - startTime; //in ms
    // strip the ms
    timeDiff /= 1000;

    // get seconds
    var seconds =timeDiff;
    console.log(seconds + " seconds");
    alert(seconds + " seconds")
}

start()
// debugger;
wasm.test_doublets(10000000)
// debugger;
end()