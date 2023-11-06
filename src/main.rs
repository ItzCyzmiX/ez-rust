mod ez;

use ez::io::Colors;

fn main() {

    println!(
        "{}{}Hello{}dd", 
        
        Colors::bg_red(),
        Colors::fg_white(), 
        Colors::fg_reset()
    );
}
