use pretty_panic::pretty_panic;
use std::panic::PanicInfo;

fn main() {
    pretty_panic!(my_panic);

    panic!("a cool panic");
}

fn my_panic(_: &PanicInfo) -> ! {
    eprintln!("a cool panic");
    loop { std::thread::sleep(std::time::Duration::from_secs(5)); }
}
