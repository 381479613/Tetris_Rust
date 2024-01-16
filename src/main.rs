// use std::num::NonZeroU32;
// use std::rc::Rc;

// use std::thread;
// use std::time;
// use winit::{
//     event::{ElementState,Event,KeyEvent,WindowEvent},
//     event_loop::{ControlFlow,EventLoop},
//     keyboard::{Key,NamedKey},
//     window::WindowBuilder,
// };

// #[derive(Debug,Clone,Copy,PartialEq,Eq)]
// enum Mode {
//     Wait,
//     WaitUntil,
//     Poll,
// }

mod util;
mod render;
fn main() {
    // let event_loop = EventLoop::new().unwrap();
    // let window = Rc::new(WindowBuilder::new()
    //     .with_title("Main Window")
    //     .build(&event_loop)
    //     .unwrap());
    // let context = softbuffer::Context::new(window.clone()).unwrap();
    // let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    // let mut mode = Mode::Wait;
    // let mut request_redraw = false;
    // let mut wait_cancelled = false;
    // let mut close_requested = false;
    // event_loop.run(move |event, elwt| {
    //     //elwt.set_control_flow(ControlFlow::Wait);
    //     use winit::event::StartCause;
    //     println!("{event:?}");

    //     match event {
    //         Event::NewEvents(start_cause) => {
    //             wait_cancelled = match start_cause {
    //                 StartCause::WaitCancelled { .. } => mode == Mode::WaitUntil,
    //                 _ => false,
    //             }
    //         }

    //         Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested } if window_id == window.id() => {
    //             let (width, height) = {
    //                 let size = window.inner_size();
    //                 (size.width, size.height)
    //             };
    //             surface
    //                 .resize(
    //                     NonZeroU32::new(width).unwrap(),
    //                     NonZeroU32::new(height).unwrap(),
    //                 )
    //                 .unwrap();

    //             let mut buffer = surface.buffer_mut().unwrap();
    //             for index in 0..(width * height) {
    //                 let y = index / width;
    //                 let x = index % width;
    //                 let red = x % 255;
    //                 let green = y % 255;
    //                 let blue = (x * y) % 255;

    //                 buffer[index as usize] = blue | (green << 8) | (red << 16);
    //             }

    //             buffer.present().unwrap();
    //         }
    //         Event::WindowEvent {
    //             event:WindowEvent::CloseRequested,
    //             window_id,
    //         } if window_id == window.id() => {
    //             elwt.exit();
    //         }
    //         _ => {}
    //     }
    // }).unwrap();

    if let Err(e) = render::run() {
        eprintln!("Error occurred: {}",e);
    }
    

}