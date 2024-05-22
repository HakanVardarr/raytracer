/*
© 2024 Hakan Vardar
Licensed under the MIT License. See LICENSE file in the project root for full license information.
*/

use raytracing::run;

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("[ERROR]: {e}")
        }
    }
}
