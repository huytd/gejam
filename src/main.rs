#![feature(conservative_impl_trait, link_args)]

extern crate libc;
pub mod js;

use std::time;
use js::{JSElement, Canvas};

const BG_COLOR: &str = "#3498db";

fn main() {

    let body = js::get_body();
    let (mut width, mut height) = (body.get_client_width(), body.get_client_height());

    js::hook_events("main");
    let canvas = Canvas::get_by_id("main").unwrap();
    let ctx = canvas.get_context_2d();
    ctx.set_image_smoothing(false);

    let mut last_frame = time::Instant::now();

    canvas.set_width(width);
    canvas.set_height(height);

    ctx.set_fill_style(BG_COLOR);
    ctx.fill_rect(0, 0, width, height);
    
    js::main_loop(move || {
        let start = time::Instant::now();
        let diff = start.duration_since(last_frame);
        last_frame = start;
        let delta =
            (diff.as_secs() * 1_000_000_000 + diff.subsec_nanos() as u64) as f64 / (1_000_000_000.0 / 60.0);
        let delta = delta.min(3.0);

        let (cur_width, cur_height) = (body.get_client_width(), body.get_client_height());
        if width != cur_width || height != cur_height {
            width = cur_width;
            height = cur_height;
            canvas.set_width(width);
            canvas.set_height(height);
            // resize
        }

        ctx.set_fill_style(BG_COLOR);
        ctx.fill_rect(0, 0, width, height);

        ctx.save();

        ctx.restore();
    });
}
