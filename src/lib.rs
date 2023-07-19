use wasm_bindgen::prelude::*;
use web_sys::console;
//use wasm_bindget::JsCast;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    // context.fill();
    sierpinski(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], 2);
    // Your code goes here!
    console::log_1(&JsValue::from_str("Done son!"));

    Ok(())
}

fn sierpinski(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3], depth: u8) {
    draw_triangle(&context, points);
    let depth = depth - 1;
    if depth > 0 {
        let [top, left, right] = points;
        let left_middle = ((top.0 + left.0) / 2.0, (top.1 + left.1) / 2.0);
        let right_middle = ((top.0 + right.0) / 2.0, (top.1 + right.1) / 2.0);
        let bottom_middle = (top.0, right.1);
        sierpinski(&context, [top, left_middle, right_middle], depth);
        sierpinski(&context, [left_middle, left, bottom_middle], depth);
        sierpinski(&context, [right_middle, bottom_middle, right], depth);
    }
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3]) {
    let [top, left, right] = points;
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke();
}
