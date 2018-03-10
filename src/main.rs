extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    CanvasRenderingContext2d
};


use stdweb::web::html_element::CanvasElement;

fn main() {
    stdweb::initialize();

    let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();

    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    context.set_fill_style_color("red");
    context.fill_rect(10.0, 10.0 , 3.0, 2.0);
    context.fill_rect(14.0, 10.0 , 3.0, 2.0);
    context.fill_rect(18.0, 10.0 , 3.0, 2.0);
}
