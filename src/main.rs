#![feature(in_band_lifetimes)]

use cellular_automata_comonad::Strip;

fn main() {
    let rule = |s: &Strip<bool>| -> bool {
        let l = s.get(-1).extract();
        let m = s.get(0).extract();
        let r = s.get(1).extract();

        match (l, m, r) {
            (true, false, false) | (false, false, true) => true,
            (_, _, _) => false,
        }
    };

    let mut data = vec![false; 50];
    data[25] = true;

    let mut layer = Strip::new(data, 0);

    for _ in 0..30 {
        let pretty: String = layer.iter().map(|i| if *i { "██" } else { "  " }).collect();
        println!("{}", pretty);
        layer = layer.extend(rule);
    }
}
