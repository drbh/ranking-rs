use ranking::{bt, calculate_metric_pairs, PairMetric};

fn main() {
    let pairs = vec![
        ("b", "c"),
        ("c", "b"),
        ("a", "b"),
        ("a", "b"),
        ("a", "d"),
        ("b", "d"),
        ("c", "e"),
        ("c", "e"),
        ("c", "e"),
    ];

    let winners = calculate_metric_pairs(pairs.clone(), PairMetric::Win);
    println!("{:?}", winners);

    let losers = calculate_metric_pairs(pairs.clone(), PairMetric::Loss);
    println!("{:?}", losers);

    let idk = calculate_metric_pairs(pairs.clone(), PairMetric::WinMinusLoss);
    println!("{:?}", idk);

    let byopps = calculate_metric_pairs(pairs.clone(), PairMetric::WinMinusLossByOpps);
    println!("{:?}", byopps);

    let est_victories = bt(pairs.clone());
    println!("{:?}", est_victories);
}
