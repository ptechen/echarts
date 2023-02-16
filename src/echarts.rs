use wasm_bindgen::prelude::*;
use web_sys::Element;

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