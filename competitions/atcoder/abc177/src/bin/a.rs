use proconio::input;

fn main() {
    input! {
        d: f32,
        t: f32,
        s: f32,
    }

    println!("{}", if d / s <= t { "Yes" } else { "No" });
}