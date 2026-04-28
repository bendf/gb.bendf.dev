import init, {
  say_hello,
  render_frame,
  get_screen_data,
  load_boot_rom,
  load_checkerboard_rom,
  load_window_rom,
  set_scx,
  set_scy,
  set_wx,
  set_wy,
  run,
} from "./wasm/squaregb.js";

function render_screen() {
  const scx = document.getElementById("scx").value;
  const scy = document.getElementById("scy").value;

  const wx = document.getElementById("wx").value;
  const wy = document.getElementById("wy").value;

  set_scx(scx);
  set_scy(scy);

  set_wx(wx);
  set_wy(wy);
  render_frame();
  const screenData = get_screen_data();
  const canvas = document.getElementById("squaregb-screen");
  const ctx = canvas.getContext("2d");
  const imageData = new ImageData(screenData, 160, 144, {
    colorSpace: "srgb",
    pixelFormat: "rgba-unorm8",
  });
  ctx.putImageData(imageData, 0, 0);
}

async function setup() {
  await init();

  document.getElementById("run-wasm-test").addEventListener("click", (e) => {
    document.getElementById("wasm-console").innerText = say_hello();
  });

  document
    .getElementById("load-checkerboard-rom")
    .addEventListener("click", (e) => {
      load_checkerboard_rom();
      console.log("Checkerboard rom loaded!");
    });
  document.getElementById("load-window-rom").addEventListener("click", (e) => {
    load_window_rom();
    console.log("Window rom loaded!");
  });
  document
    .getElementById("button-render-screen")
    .addEventListener("click", (e) => {
      render_screen();
      console.log("Screen rendered manually!");
    });

  load_boot_rom();
  render_screen();
}

setup();
