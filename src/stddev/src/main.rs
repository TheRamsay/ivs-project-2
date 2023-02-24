use mathlib::add;

fn main() {
    // dummy calculation
    let _data = (0..100).map(|i| add(i, 1))
        .collect::<Vec<_>>();
}
