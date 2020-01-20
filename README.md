# Ranking

Simple consolidated ranking from many samples of ranked data. No dependencies, 100% Rust, very simple, lightweight and fast.

## Use Crate
```toml
ranking = "0.0.1"
```

## Ranking Algorithims

### Positional
- Mean position
- Median position

### Relational

#### Aggregation
- Wins
- Losses
- Wins Minus Losses 

#### Iterative
- Bradley Terry Model


## Example Code

The following code uses both the `Median` and `Average` metric to compute the final ranking. This code is also in `examples/basic.rs`

```rust
use ranking::{calculate_ranking, Metric};

fn main() {
    // example of people ranking tequila brands
    let ranking_a = vec!["Don Julio", "Patron", "Herradura", "Espolon", "El Jimidor"];
    let ranking_b = vec!["Espolon", "Herradura", "Don Julio", "El Jimidor", "Patron"];
    let ranking_c = vec!["Espolon", "Don Julio", "El Jimidor", "Herradura", "Patron"];

    let everyones_rankings = vec![
        ("david", ranking_a),
        ("sakura", ranking_b),
        ("joe", ranking_c),
    ];

    // here we use the median to calculate the final ranking
    let m_metric = Metric::Median;
    let rankings_by_median = calculate_ranking(everyones_rankings.clone(), m_metric);
    println!("{:?}", rankings_by_median);
    // [(0.0, "Espolon"), (1.0, "Don Julio"), (2.0, "Herradura"), (3.0, "El Jimidor"), (4.0, "Patron")]

    // here we use the average to calculate the final ranking
    let a_metric = Metric::Average;
    let rankings_by_average = calculate_ranking(everyones_rankings.clone(), a_metric);
    println!("{:?}", rankings_by_average);
    // [(1.0, "Don Julio"), (1.0, "Espolon"), (2.0, "Herradura"), (3.0, "El Jimidor"), (3.0, "Patron")]
}
```


```rust
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

    let est_victories  = bt(pairs.clone());
    println!("{:?}", est_victories);
    // [(0.44185534, "a"), (0.20987698, "c"), (0.13296455, "b"), (0.052943442, "d"), (0.04120922, "e")]
}
```