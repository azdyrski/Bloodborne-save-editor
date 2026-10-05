import init, * as wasm from "../wasm/bb_save_core.js";

let ready;
const ensureReady = () => (ready ??= init());

const toMessage = (e) => (typeof e === "string" ? e : (e?.message ?? String(e)));

// Same calling convention as Tauri's invoke: command name plus camelCase args.
export async function invoke(cmd, args = {}) {
  await ensureReady();
  try {
    return JSON.parse(wasm.invoke(cmd, JSON.stringify(args)));
  } catch (e) {
    throw toMessage(e);
  }
}

export async function loadSave(file) {
  await ensureReady();
  const bytes = new Uint8Array(await file.arrayBuffer());
  try {
    return JSON.parse(wasm.load_save(bytes));
  } catch (e) {
    throw toMessage(e);
  }
}

export async function getSaveBytes() {
  await ensureReady();
  try {
    return wasm.get_save_bytes();
  } catch (e) {
    throw toMessage(e);
  }
}

export async function exportAppearance() {
  await ensureReady();
  try {
    return wasm.export_appearance_bytes();
  } catch (e) {
    throw toMessage(e);
  }
}

export async function importAppearance(file) {
  await ensureReady();
  const bytes = new Uint8Array(await file.arrayBuffer());
  try {
    return wasm.import_appearance_bytes(bytes);
  } catch (e) {
    throw toMessage(e);
  }
}
