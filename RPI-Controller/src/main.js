const { invoke } = window.__TAURI__.core;

let ipInput;
let status;

async function connect_to_ip() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  status.textContent = await invoke("connect", { ipaddr: ipInput.value });
}

window.addEventListener("DOMContentLoaded", () => {
  ipInput = document.querySelector("#ip-input");
  status = connect_to_ip();
});
