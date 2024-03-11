// 胶水代码 用来与tauri交互

const Swal = window.Swal;
const AwesomeQR = window.AwesomeQR.AwesomeQR;

export async function connectWallet() {
  console.log("invoke tauri connectWallet");
  await window.__TAURI__?.invoke("connect_wallet");
  setTimeout(async () => {
    alert("send_tx");
    await window.__TAURI__?.invoke("send_tx");
  }, 2000);
}

export async function sendTx() {
  console.log("invoke tauri sendTx");
  return await window.__TAURI__?.invoke("send_tx");
}

export async function newQR(message) {
  const buff = new AwesomeQR({
    text: message,
    size: 500,
  }).draw();
  Swal.fire({
    html: `<img src="${buff}" />`,
    showConfirmButton: false,
  });
}

window.__TAURI__?.event.listen("send_qr", event => {
  const { message } = event.payload;
  console.log("send_qr message =>", message);
  newQR(message);
});

export function warning(title) {
  notify("warning", title);
}

export function error(title) {
  notify("error", title);
}

export function success(title) {
  notify("success", title);
}

export function info(title) {
  notify("info", title);
}

export function question(title) {
  notify("question", title);
}

function notify(icon, title) {
  Swal.fire({
    position: "top-end",
    icon,
    title,
    showConfirmButton: false,
    timer: 3000,
  });
}
