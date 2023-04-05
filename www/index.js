import * as wasm from "wasm-game-of-life";

const buttonNormal = document.getElementById("button-normal");
buttonNormal.onclick = () => {
    alert("Normal Greeting");
};

const buttonWasm = document.getElementById("button-wasm");
buttonWasm.onclick = () => {
    wasm.greet();
};
