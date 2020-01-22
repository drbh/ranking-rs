use ranking::strongest_longest_path;

fn main() {
    let pairs = vec![
        // //
        // ("a", "b"),
        // ("b", "c"),


        ("A","B"),
        ("B","C"),
        ("D","B"),
        ("D","A"),
        ("D","C"),
    ];

    let est_victories = strongest_longest_path(pairs.clone());
    println!("{:?}", est_victories.clone());

    let mut rank = 0f32;
    let mut last_score = est_victories[0].0;

    let ranks: Vec<(f32, String)> = est_victories
        .into_iter()
        .map(|(score, name)| {
            if (score - last_score) > 0.00001 {
                rank = rank + 1f32;
                last_score = score
            }
            (rank, name)
        })
        .collect();

    println!("{:?}", ranks);
}
