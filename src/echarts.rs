use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    // this should be in js-sys but is not. see https://github.com/rustwasm/wasm-bindgen/issues/2865
    pub fn import(s: &str) -> js_sys::Promise;

    pub type Window;

    #[wasm_bindgen(method, getter, js_name = "wasmBindgenSnippetsPath")]
    pub fn wasm_bindgen_snippets_path(this: &Window) -> String;
}

#[wasm_bindgen(module = "/js/echarts.esm.js")]
extern "C" {
    pub fn init_charts(dom: Element, theme: Option<&str>, option: JsValue) -> ECharts;
}

#[wasm_bindgen(module = "/js/echarts.esm.js")]
extern "C" {
    pub type ECharts;
    #[wasm_bindgen(method)]
    pub fn setOption(this: &ECharts, option: JsValue);
}
