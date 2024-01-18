
mod util;
mod render;

fn main() {

    if let Err(e) = render::run() {
        eprintln!("Error occurred: {}",e);
    }
    

}