use std::cmp::Ordering::Equal;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Metric {
    Median,
    Average,
}

pub fn calculate_ranking(
    all_rankings: Vec<(&str, Vec<&str>)>,
    metric: Metric,
) -> Vec<(f32, String)> {
    let mut rank_options: HashMap<String, Vec<usize>> = HashMap::new();

    for label_and_scores in all_rankings {
        let mut counter = 0;
        let (_label, score) = label_and_scores;
        for s in score {
            rank_options
                .entry(String::from(s))
                .or_insert_with(Vec::new)
                .push(counter);
            counter = counter + 1;
        }
    }

    let mut results: Vec<(f32, String)> = vec![];
    match metric {
        Metric::Median => {
            println!("Ranking Metric: {}", "Median");
            for (k, v) in rank_options.iter() {
                results.push((median(v), String::from(k)));
            }
        }
        Metric::Average => {
            println!("Ranking Metric: {}", "Average");
            for (k, v) in rank_options.iter() {
                results.push((average(v), String::from(k)));
            }
        }
    }
    // use this to sort floats
    results.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    results
}

// convert to ints then floats based on rank indexs
fn average(imm_numbers: &Vec<usize>) -> f32 {
    let numbers = imm_numbers
        .into_iter()
        .map(|i| *i as i32)
        .collect::<Vec<i32>>();

    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

// sort and return the mid point value
fn median(imm_numbers: &Vec<usize>) -> f32 {
    let mut numbers = imm_numbers.to_vec();
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid] as f32
}

// // should write some tests!
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
