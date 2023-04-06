const invoke = window.__TAURI__.invoke;

export async function invokeParseAndEval(name) {
    return await invoke("parse_and_eval", {name: name});
}