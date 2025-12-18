use how_sorted::{
    longest_increasing_subsequence_length, score_by_exact_placement, score_by_lis,
    score_by_num_modifications_slow,
};
use quickcheck::{TestResult, quickcheck};

fn make_permutation(mut indices: Vec<usize>) -> Vec<i32> {
    // Cap at 6 elements for performance - A* with h=0 is exponential!
    let n = indices.len().min(6);
    if n == 0 {
        return vec![];
    }

    // Normalize indices to create a valid permutation of 0..n
    for i in 0..n {
        indices[i] = indices[i] % (n - i);
    }

    let mut available: Vec<i32> = (0..n as i32).collect();
    let mut result = Vec::with_capacity(n);

    for i in 0..n {
        let idx = indices[i] % available.len();
        result.push(available.remove(idx));
    }

    result
}

#[test]
fn test_lis_simple_cases() {
    assert_eq!(longest_increasing_subsequence_length(&[0, 1, 2, 3]), 4);
    assert_eq!(longest_increasing_subsequence_length(&[3, 2, 1, 0]), 1);
    // [1, 2, 3, 4, 5, 0] - the 0 is at the end, LIS is [1,2,3,4,5] = 5
    assert_eq!(
        longest_increasing_subsequence_length(&[1, 2, 3, 4, 5, 0]),
        5
    );
    assert_eq!(longest_increasing_subsequence_length(&[1, 0]), 1);
}

#[test]
fn test_score_by_lis_simple_cases() {
    // Already sorted - 0 moves needed
    assert_eq!(score_by_lis(&[0, 1, 2, 3]), 0);

    // Fully reversed - need n-1 moves
    assert_eq!(score_by_lis(&[3, 2, 1, 0]), 3);

    // One element out of place - [1,2,3,4,5,0] needs 1 move (move 0 to front)
    assert_eq!(score_by_lis(&[1, 2, 3, 4, 5, 0]), 1);
    assert_eq!(score_by_lis(&[1, 0]), 1);
}

#[test]
fn test_lis_heuristic_matches_oracle_simple() {
    let cases: Vec<Vec<i32>> = vec![
        vec![0, 1, 2, 3],
        vec![3, 2, 1, 0],
        vec![1, 0],
        vec![1, 2, 3, 4, 5, 0],
        vec![0],
        vec![],
    ];

    for case in cases {
        let l: im::Vector<i32> = case.iter().cloned().collect();
        let oracle_cost = score_by_num_modifications_slow(l.clone());
        let lis_estimate = score_by_lis(&case);

        assert_eq!(
            oracle_cost,
            Some(lis_estimate),
            "Mismatch for {:?}: oracle={:?}, lis={}",
            case,
            oracle_cost,
            lis_estimate
        );
    }
}

#[test]
fn test_lis_heuristic_is_admissible() {
    fn prop(indices: Vec<usize>) -> TestResult {
        let perm = make_permutation(indices);
        if perm.is_empty() {
            return TestResult::discard();
        }

        let l: im::Vector<i32> = perm.iter().cloned().collect();
        let oracle_cost = match score_by_num_modifications_slow(l) {
            Some(c) => c,
            None => return TestResult::discard(),
        };
        let lis_estimate = score_by_lis(&perm);

        // Admissible means: estimate <= true cost
        TestResult::from_bool(lis_estimate <= oracle_cost)
    }

    quickcheck(prop as fn(Vec<usize>) -> TestResult);
}

#[test]
fn test_lis_heuristic_is_exact() {
    fn prop(indices: Vec<usize>) -> TestResult {
        let perm = make_permutation(indices);
        if perm.is_empty() {
            return TestResult::discard();
        }

        let l: im::Vector<i32> = perm.iter().cloned().collect();
        let oracle_cost = match score_by_num_modifications_slow(l) {
            Some(c) => c,
            None => return TestResult::discard(),
        };
        let lis_estimate = score_by_lis(&perm);

        // The LIS heuristic should be exactly equal to the true cost
        TestResult::from_bool(lis_estimate == oracle_cost)
    }

    quickcheck(prop as fn(Vec<usize>) -> TestResult);
}

#[test]
fn test_exact_placement_is_not_admissible() {
    // This test demonstrates that score_by_exact_placement is NOT admissible
    // [1, 2, 3, 4, 5, 0] - move 0 to the front to get [0, 1, 2, 3, 4, 5]
    let case = vec![1, 2, 3, 4, 5, 0];
    let l: im::Vector<i32> = case.iter().cloned().collect();

    let oracle_cost = score_by_num_modifications_slow(l).unwrap();
    let exact_placement_estimate = score_by_exact_placement(case.iter().cloned());

    // exact_placement says 6 (all elements are wrong), but true cost is 1
    assert_eq!(oracle_cost, 1);
    assert_eq!(exact_placement_estimate, 6);
    assert!(
        exact_placement_estimate > oracle_cost,
        "exact_placement overestimates!"
    );
}
