use pretty_panic::pretty_panic;

fn main() {
    pretty_panic!();

    panic!("explicit call to panic!()");
}
