
mod util;
mod render;
mod block;

fn main() {

    if let Err(e) = render::run() {
        eprintln!("Error occurred: {}",e);
    }
    

}