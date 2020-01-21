#![allow(unused_variables)]

use std::cmp::Ordering::Equal;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Metric {
    Median,
    Average,
}

#[derive(Debug)]
pub enum PairMetric {
    Win,
    Loss,
    WinMinusLoss,
    WinMinusLossByOpps,
}

pub fn bt(
    all_pairs: Vec<(&str, &str)>,
    // metric: PairMetric,
) -> Vec<(f32, String)> {
    let mut num_games: HashMap<(&str, &str), i32> = HashMap::new();
    let mut win_table: HashMap<String, f32> = HashMap::new();
    for pair in &all_pairs {
        // update the win freq table
        let uni_key: (&str, &str);
        if pair.1 < pair.0 {
            uni_key = (pair.1, pair.0);
        } else {
            uni_key = (pair.0, pair.1);
        }

        num_games
            .get_mut(&uni_key)
            .map(|count| {
                *count += 1;
            })
            .unwrap_or_else(|| {
                num_games.insert(uni_key, 1);
            });

        win_table
            .get_mut(pair.0)
            .map(|count| {
                *count += 1f32;
            })
            .unwrap_or_else(|| {
                win_table.insert(pair.0.to_owned(), 1f32);
            });

        // not sure if this map is a good idea
        // should insert a 0 but do nothing else
        win_table.get_mut(pair.1).map(|_| {}).unwrap_or_else(|| {
            win_table.insert(pair.1.to_owned(), 0f32);
        });
    }

    let mut num_dummy_wins = 0;
    for (k, v) in win_table.iter_mut() {
        num_dummy_wins = num_dummy_wins + 1;
        *v += 1f32;
    }

    win_table.insert("_DUMMY".to_string(), num_dummy_wins as f32);

    let wt = win_table.clone();

    let mut unique_players: Vec<String> = vec![];
    for (k, v) in win_table.iter_mut() {
        unique_players.push(String::from(k));
        num_games.insert(("_DUMMY", k), 2);
    }

    unique_players.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    let intial_score = 1.0 / unique_players.len() as f32;

    let mut ranks: Vec<f32> = vec![];
    for _ in 0..unique_players.len() {
        ranks.push(intial_score.clone())
    }

    for iters in 0..1000 {
        let oldranks = ranks.clone();
        let mut ply_pos = 0;
        for player in unique_players.clone() {
            let mut denom = 0f32;
            for p in unique_players.clone() {
                if player != p {
                    let uni_key: (&str, &str);
                    if player < p {
                        uni_key = (&player, &p);
                    } else {
                        uni_key = (&p, &player);
                    }
                    let n_count = num_games.get(&uni_key).unwrap_or(&0);

                    // score
                    let p_pos = unique_players.iter().position(|r| *r == p).unwrap();
                    let score = ranks[p_pos] + ranks[ply_pos];
                    let den = *n_count as f32 / score;
                    denom = denom + den;
                }
            }

            let numerator = wt.get(&player).unwrap_or(&1f32);
            let new_v = numerator / denom;

            ranks[ply_pos] = new_v;
            ply_pos = ply_pos + 1;
        }
        let rank_sum: f32 = ranks.iter().sum();

        ranks = ranks
            .iter_mut()
            .map(|val| {
                return *val / rank_sum;
            })
            .collect();

        let new_rank_sum = ranks.clone(); 
        let mut old_rank_sum = oldranks.clone(); 
        let mut err_tot = 0f32;
        for it in new_rank_sum.iter().zip(old_rank_sum.iter_mut()) {
            let (ai, bi) = it;
            err_tot = err_tot + (*ai - *bi).abs()
        }

        if err_tot < 0.000_000_1 {
            // println!("{:?}", "Converged!");
            break;
        }
    }
    let mut results: Vec<(f32, String)> = vec![];
    for i in 1..unique_players.len(){
        results.push((ranks[i], unique_players[i].clone()))
    }
    // println!("{}", "Ranking Bradley Terry Model");
    results.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Equal));
    results
}

pub fn calculate_metric_pairs(
    all_pairs: Vec<(&str, &str)>,
    metric: PairMetric,
) -> Vec<(f32, String)> {
    let mut freq_counts: HashMap<String, i32> = HashMap::new();
    let mut freq_table: HashMap<String, HashMap<String, bool>> = HashMap::new();
    for pair in &all_pairs {
        freq_table
            .get_mut(pair.0)
            .map(|valll| {
                valll.get_mut(pair.1).map(|_| {}).unwrap_or_else(|| {
                    valll.insert(pair.1.to_owned(), true);
                });
            })
            .unwrap_or_else(|| {
                freq_table.insert(pair.0.to_owned(), HashMap::new());
            });

        freq_table
            .get_mut(pair.1)
            .map(|valll| {
                valll.get_mut(pair.0).map(|_| {}).unwrap_or_else(|| {
                    valll.insert(pair.0.to_owned(), true);
                });
            })
            .unwrap_or_else(|| {
                freq_table.insert(pair.1.to_owned(), HashMap::new());
            });
    }
    for (k, v) in freq_table.iter() {
        freq_counts
            .entry(String::from(k))
            .or_insert(v.keys().len() as i32);
    }

    // println!("{:#?}", freq_counts);

    let mut win_table: HashMap<String, usize> = HashMap::new();
    for pair in &all_pairs {
        // update the win freq table
        win_table
            .get_mut(pair.0)
            .map(|count| {
                *count += 1;
            })
            .unwrap_or_else(|| {
                win_table.insert(pair.0.to_owned(), 1);
            });

        // not sure if this map is a good idea
        // should insert a 0 but do nothing else
        win_table.get_mut(pair.1).map(|_| {}).unwrap_or_else(|| {
            win_table.insert(pair.1.to_owned(), 0);
        });
    }

    let mut loss_table: HashMap<String, usize> = HashMap::new();
    for pair in all_pairs {
        // update the win freq table
        loss_table
            .get_mut(pair.1)
            .map(|count| {
                *count += 1;
            })
            .unwrap_or_else(|| {
                loss_table.insert(pair.1.to_owned(), 1);
            });

        // not sure if this map is a good idea
        // should insert a 0 but do nothing else
        loss_table.get_mut(pair.0).map(|_| {}).unwrap_or_else(|| {
            loss_table.insert(pair.0.to_owned(), 0);
        });
    }
    // println!("{:?}", win_table);
    // println!("{:?}", loss_table);

    let mut results: Vec<(f32, String)>;
    match metric {
        PairMetric::Win => {
            println!("Ranking PairMetric: {}", "Wins");
            results = win_table
                .iter()
                .map(|(key, value)| return (*value as f32, key.to_string()))
                .collect();
            // println!("{:?}", wins);

            // for (k, v) in rank_options.iter() {
            //     results.push((median(v), String::from(k)));
            // }
            ()
        }
        PairMetric::Loss => {
            println!("Ranking PairMetric: {}", "Losses");
            results = loss_table
                .iter()
                .map(|(key, value)| return (*value as f32, key.to_string()))
                .collect();
            ()
        }
        PairMetric::WinMinusLoss => {
            println!("Ranking WinMinusLoss: {}", "Adv");

            results = loss_table
                .iter()
                .map(|(key, value)| {
                    let w_l = *win_table.get(&key.to_string()).unwrap_or(&0) as i32 - *value as i32;
                    return (w_l as f32, key.to_string());
                })
                .collect();

            ()
        }
        PairMetric::WinMinusLossByOpps => {
            println!("Ranking WinMinusLossByOpps: {}", "Adv");

            let mut min_seen = 99999;

            let win_minus_loss_int_table: Vec<(String, i32)> = loss_table
                .iter()
                .map(|(key, value)| {
                    let w_l = *win_table.get(&key.to_string()).unwrap_or(&0) as i32 - *value as i32;

                    if w_l < min_seen {
                        min_seen = w_l.clone();
                    };

                    return (key.to_string(), w_l);
                })
                .collect();

            results = win_minus_loss_int_table
                .iter()
                .map(|(key, value)| {
                    let op_count = freq_counts.get(key).unwrap();
                    let calculated_score = *value as f32 / *op_count as f32;
                    return (calculated_score as f32, key.to_string());
                })
                .collect();

            // for tup in win_minus_loss_int_table {
            //     let (key, value) = tup;
            //     // let ops = freq_table.get(&key).unwrap();
            //     // println!("{:?}", ops);
            //     println!("{} {}", key, value + min_seen.abs());
            // }

            // println!("{:?}", win_minus_loss_int_table);

            ()
        }
    }
    // // use this to sort floats
    results.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Equal));
    results
}

// https://en.wikipedia.org/wiki/Mean_reciprocal_rank
pub fn calculate_metric(
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
