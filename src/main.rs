mod util;
mod render;
mod block;
mod input;

fn main() {

    if let Err(e) = render::run() {
        eprintln!("Error occurred: {}",e);
    }
    

}