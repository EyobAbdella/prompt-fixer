import init, { test } from "./pkg/PromptFixer.js";

const button = document.getElementById("btn");
const output = document.getElementById("output");


let initialized = false;

async function intiWasm(){
  if (!initialized){
    await init();
    initialized = true;
  }

}

button.addEventListener("click", async () => {
  await intiWasm();
  output.textContent = test();
})



