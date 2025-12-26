use how_sorted::*;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
)]
enum Outcome {
  First,
  Second,
  Tie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ComparisonResult {
  sequence_a: Vec<i32>,
  sequence_b: Vec<i32>,
  exact_placement: Outcome,
  distance: Outcome,
  correct_neighbors: Outcome,
  runs: Outcome,
  lis: Outcome,
}

fn generate_random_sequence(
  rng: &mut impl rand::Rng,
) -> Vec<i32> {
  let mut seq: Vec<i32> = (0..10).collect();
  seq.shuffle(rng);
  seq
}

fn compare_scores<T: Ord>(
  score_a: T,
  score_b: T,
) -> Outcome {
  // Lower scores are better (less unsorted)
  match score_a.cmp(&score_b) {
    std::cmp::Ordering::Less => Outcome::First,
    std::cmp::Ordering::Greater => Outcome::Second,
    std::cmp::Ordering::Equal => Outcome::Tie,
  }
}

fn main() {
  println!(
    "Generating 100,000 pairs of random sequences..."
  );

  let mut rng = rand::rngs::StdRng::seed_from_u64(42); // Fixed seed for reproducibility
  let mut results = Vec::with_capacity(100_000);

  let mut i = 0;
  loop {
    if i % 10_000 == 0 {
      println!("Progress: {}/100,000", i);
    }

    let seq_a = generate_random_sequence(&mut rng);
    let seq_b = generate_random_sequence(&mut rng);

    // Run all heuristics on both sequences
    let exact_a =
      score_by_exact_placement(seq_a.iter().copied());
    let exact_b =
      score_by_exact_placement(seq_b.iter().copied());

    let distance_a =
      score_by_distance(seq_a.iter().copied());
    let distance_b =
      score_by_distance(seq_b.iter().copied());

    let neighbors_a = score_by_correct_neighbors(&seq_a);
    let neighbors_b = score_by_correct_neighbors(&seq_b);

    let runs_a = score_by_runs(&seq_a);
    let runs_b = score_by_runs(&seq_b);

    let lis_a = score_by_lis(&seq_a);
    let lis_b = score_by_lis(&seq_b);

    //if !(lis_a <= 3 || lis_b <= 3) {
    if false {
      continue;
    }

    results.push(ComparisonResult {
      sequence_a: seq_a,
      sequence_b: seq_b,
      exact_placement: compare_scores(exact_a, exact_b),
      distance: compare_scores(distance_a, distance_b),
      correct_neighbors: compare_scores(
        neighbors_a,
        neighbors_b,
      ),
      runs: compare_scores(runs_a, runs_b),
      lis: compare_scores(lis_a, lis_b),
    });

    i += 1;
    if i == 100_000 {
      break;
    }
  }

  println!(
    "Saving results to heuristic_comparison_results.json..."
  );
  let json =
    serde_json::to_string_pretty(&results).unwrap();
  let mut file =
    File::create("heuristic_comparison_results.json")
      .unwrap();
  file.write_all(json.as_bytes()).unwrap();

  println!(
    "Done! Results saved to heuristic_comparison_results.json"
  );
  println!("Total comparisons: {}", results.len());
}
