use std::time::Instant;

use buoy::prelude::*;
use buoy::render::CommandList;

mod ui;
#[allow(unused_imports)]
use ui::{GridRepeating, Repeating};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut first_frame = true;
    let window = video_subsystem
        .window("rust-sdl2 demo", 1920, 1080)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut ctx = Window::default();
    let mut ui_commands = CommandList::default();

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Get the window size
        let window_size = canvas.output_size().unwrap();

        // Get the mouse state
        let mouse_state = MouseState::new(&event_pump);

        ui_commands.clear();
        build_ui(
            &mut ui_commands,
            window_size.0 as f32,
            window_size.1 as f32,
            &mut ctx,
            &mut first_frame,
        );

        // Render the UI
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        render_ui(&mut ctx, &mut canvas, &mut ui_commands, mouse_state);
        canvas.present();
    }
}

fn render_ui(
    window: &mut Window,
    canvas: &mut WindowCanvas,
    commands: &mut CommandList,
    mouse: MouseState,
) {
    // Render all of the colored quads
    for quad in commands.colored_quads.drain(..) {
        canvas.set_draw_color(Color::RGBA(
            quad.color.red(),
            quad.color.green(),
            quad.color.blue(),
            quad.color.alpha(),
        ));
        let rect = Rect::new(
            quad.quad.x as i32,
            quad.quad.y as i32,
            quad.quad.width as u32,
            quad.quad.height as u32,
        );
        canvas.fill_rect(rect).unwrap();
    }

    // Handle all of the hover quads
    let mouse_x = mouse.x() as f32;
    let mouse_y = mouse.y() as f32;
    for quad in commands.hover_quads.drain(..) {
        // Make sure x is within range
        if quad.quad.x > mouse_x || quad.quad.x + quad.quad.width < mouse_x {
            continue;
        }

        // Make sure y is within range
        if quad.quad.y > mouse_y || quad.quad.y + quad.quad.height < mouse_y {
            continue;
        }

        // Write the message
        window.write_message(quad.message, ());
    }
}

fn build_ui(
    commands: &mut CommandList,
    window_width: f32,
    window_height: f32,
    ctx: &mut Window,
    first_frame: &mut bool,
) {
    let window_region = Region {
        pos: Point::zero(),
        area: Area {
            width: window_width,
            height: window_height,
        },
    };

    // Create a fader for one of the widgets
    if *first_frame {
        // let fader = Fader::new(element::Id::str("BlueBox_2").append_str("border"));
        // ctx.filter(Rc::new(fader));

        // let grower = Grower::new(element::Id::str("BlueBox_2").append_str("inner"));
        // ctx.filter(Rc::new(grower));
        *first_frame = false;
    }

    let render_start = Instant::now();

    let elem_obj = ctx.run(window_region.area, GridRepeating, FilterStack::default());
    elem_obj.render(window_region, commands);

    println!(
        "Generated rendering commands in {} μs",
        render_start.elapsed().subsec_micros()
    );
}
