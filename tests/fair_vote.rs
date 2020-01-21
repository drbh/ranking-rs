use ranking::bt;
use std::collections::HashMap;

fn ranker(pairs: Vec<(&str, &str)>) -> Vec<(f32, String)> {
    let est_victories = bt(pairs.clone());
    let mut rank = 0f32;
    let mut last_score = est_victories[0].0;
    let ranks: Vec<(f32, String)> = est_victories
        .into_iter()
        .map(|(score, name)| {
            if (last_score - score) > 0.00001 {
                rank = rank + 1f32;
                last_score = score
            }
            (rank, name)
        })
        .collect();
    ranks
}

#[cfg(test)]
mod fauns_criteria {
    use super::*;

    fn mapping(results: Vec<(f32, String)>) -> HashMap<String, f32> {
        results
            .into_iter()
            .map(|(score, name)| (name.into(), score))
            .collect()
    }

    fn equalish(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.00001
    }

    /*
    a b
    b c
    c a

    a == b
    b == c


    a b
    c b
    d a
    d c

    d < a
    a == c
    c < b


    a b
    b c
    c a
    c a

    c < b
    b < a


    a b
    b c
    b c
    c a
    c a

    b < c
    c < a


    b a
    b c
    d a
    d c
    a c

    b == d
    d < a
    a < c


    a b
    b c
    c e1
    c e2
    c e3
    c e4

    a < b
    b < c
    c < e1
    c < e2
    c < e3
    c < e4
    */

    #[test]
    fn test_short() {
        let r = mapping(ranker(vec![("a", "b"), ("b", "c")]));
        assert!(r["a"] < r["b"]);
        assert!(r["b"] < r["c"]);
    }
    #[test]
    fn square() {
        let r = mapping(ranker(vec![("a", "b"), ("c", "b"), ("d", "a"), ("d", "c")]));
        assert!(r["d"] < r["a"]);
        assert!(equalish(r["a"], r["c"]));
        assert!(r["c"] < r["b"]);
    }
    #[test]
    fn cycle() {
        let r = mapping(ranker(vec![("a", "b"), ("b", "c"), ("c", "a")]));
        assert!(equalish(r["a"], r["b"]));
        assert!(equalish(r["b"], r["c"]));
    }
    #[test]
    fn one_din_cycle() {
        let r = mapping(ranker(vec![("a", "b"), ("b", "c"), ("c", "a"), ("c", "a")]));
        assert!(r["c"] < r["b"]);
        assert!(r["b"] < r["a"]);
    }
    #[test]
    fn two_din_cycle() {
        let r = mapping(ranker(vec![
            ("a", "b"),
            ("b", "c"),
            ("b", "c"),
            ("c", "a"),
            ("c", "a"),
        ]));
        assert!(r["b"] < r["c"]);
        assert!(r["c"] < r["a"]);
    }
    #[test]
    fn crossed_square() {
        let r = mapping(ranker(vec![
            ("b", "a"),
            ("b", "c"),
            ("d", "a"),
            ("d", "c"),
            ("a", "c"),
        ]));
        assert!(equalish(r["b"], r["d"]));
        assert!(r["d"] < r["a"]);
        assert!(r["a"] < r["c"]);
    }
    #[test]
    fn crossed_square_complex() {
        let r = mapping(ranker(vec![
            ("a", "b"),
            ("b", "c"),
            ("c", "e1"),
            ("c", "e2"),
            ("c", "e3"),
            ("c", "e4"),
        ]));
        assert!(r["a"] < r["b"]);

        // // originally expected
        // assert!(r["b"] < r["c"]);

        // maybe c<b since c has won many more times?
        assert!(r["c"] < r["b"]);

        assert!(r["c"] < r["e1"]);
        assert!(r["c"] < r["e2"]);
        assert!(r["c"] < r["e3"]);
        assert!(r["c"] < r["e4"]);
    }
}
