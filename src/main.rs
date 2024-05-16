use raytracing::run;

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("[ERROR]: {e}")
        }
    }
}
