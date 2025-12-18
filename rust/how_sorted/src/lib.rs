// This library contains several functions that can be used as heuristics for how sorted a list is.
// For simplicity, the inputs to these functions are lists of integers in the range 0 to 9
// (inclusive), with no duplicates.  This means that the correctly sorted list is always
// `[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]`.

// implementation 1 - score by exact placement:
// This function scores the input list by counting the number of elements that aren't in their
// correct location.  Because of the simplifying assumptions above, the implementation is quite
// simple:
//
// ```ocaml
// let score_by_exact_placement l =
//   List.length (List.filteri l ~f:(fun i v -> i <> v))
// ```
//
// A score of 0 is perfect, larger scores are worse
// SNIPPET_START: score_by_exact_placement
pub fn score_by_exact_placement(l: impl Iterator<Item = i32>) -> usize {
    l.enumerate().filter(|&(i, v)| i as i32 != v).count()
}
// SNIPPET_END: score_by_exact_placement

// implementation 2 - score by distance:
// This version sums up how far each element is from it's correct location.
//
// ```ocaml
// let score_by_distance l =
//   List.sum (module Int) (List.mapi l ~f:(fun i v -> Int.abs (i - v)))
// ```
//
// A score of 0 is perfect, and larger scores are worse
pub fn score_by_distance(l: impl Iterator<Item = i32>) -> i32 {
    l.enumerate().map(|(i, v)| (i as i32 - v).abs()).sum()
}

// implementation 3 - score by relative placement:
// This version compares each neighboring pair of elements and counts the number of neighbors that
// are out of order.
//
// ```ocaml
// let score_by_relative_placement l =
//    List.count (List.pairs l) ~f:(fun a b -> b < a)
// ```
//
// A score of 0 is perfect, and larger scores are worse
pub fn score_by_relative_placement(l: &[i32]) -> usize {
    l.windows(2).filter(|pair| pair[1] < pair[0]).count()
}

// implementation 3 - score by correct neighbors:
// This version compares each neighboring pair of elements and counts the number of neighbors that
// aren't actually next to one another in the sorted list.
//
// ```ocaml
// let score_by_relative_placement l =
//    List.count (List.pairs l) ~f:(fun a b -> b - 1 <> a)
// ```
//
// A score of 0 is perfect, and larger scores are worse
pub fn score_by_correct_neighbors(l: &[i32]) -> usize {
    l.windows(2).filter(|pair| pair[1] - 1 != pair[0]).count()
}

// implementation 4 - score by num modifications:
// This is a heuristic that would be easy for a human to understand, but could be hard for them to
// implement.  Imagine that you have a small hand of playing cards laid out on a table in some
// order.  We can define "how sorted are these cards" as "what is the smallest number of
// modifications to this list are necessary to get it into sorted order", where a "modification" is
// defined as picking up any card and putting it anywhere else in the list.
//
// I don't know if there's a good algorithm for computing this, so I suggest searching through the
// space of modifications using A* search.
//
// The two heuristic functions that we can use are
// 1. The "oracle" heuristic: always return 0. This causes the graph search to
//    explore all possible states.
// 2. The "longest increasing subsequence" heuristic: use LIS to determine how
//    fast close we are to a solution
pub fn score_by_num_modifications_with_heuristic(
    l: im::Vector<i32>,
    heuristic_fn: fn(&[i32]) -> usize,
) -> Option<usize> {
    use astar::SearchProblem;

    struct SortProblem {
        heuristic_fn: fn(&[i32]) -> usize,
    }

    impl SearchProblem for SortProblem {
        type Node = im::Vector<i32>;
        type Cost = usize;
        type Iter = std::vec::IntoIter<(Self::Node, Self::Cost)>;

        fn heuristic(&self, node: &Self::Node) -> Self::Cost {
            let slice: Vec<i32> = node.iter().cloned().collect();
            (self.heuristic_fn)(&slice)
        }

        fn is_end(&self, node: &Self::Node) -> bool {
            node.iter().enumerate().all(|(i, &v)| i as i32 == v)
        }

        fn neighbors(&self, node: &Self::Node, _cost: &Self::Cost) -> Self::Iter {
            let mut result = Vec::new();
            let len = node.len();

            for remove_idx in 0..len {
                let removed_value = node[remove_idx];
                let mut without = node.clone();
                without.remove(remove_idx);

                for insert_idx in 0..len {
                    if insert_idx != remove_idx {
                        let mut new_list = without.clone();
                        new_list.insert(insert_idx, removed_value);
                        result.push((new_list, 1));
                    }
                }
            }

            result.into_iter()
        }
    }

    let problem = SortProblem { heuristic_fn };

    // path.len() includes the start state, so subtract 1 to get the number of moves
    astar::astar(&problem, l).map(|(path, _total_cost)| path.len().saturating_sub(1))
}

// Oracle heuristic that always returns 0.
// This is trivially admissible and can be used to compute the true optimal cost
// for testing that other heuristics are admissible.
pub fn oracle_heuristic(_l: &[i32]) -> usize {
    0
}

// Compute the true optimal cost using the oracle (h=0) heuristic
pub fn score_by_num_modifications_slow(l: im::Vector<i32>) -> Option<usize> {
    score_by_num_modifications_with_heuristic(l, oracle_heuristic)
}

// Longest Increasing Subsequence (LIS) based heuristic.
// This computes the exact minimum number of moves needed to sort the list.
// The insight is that elements in the LIS don't need to move - everything else does.
// This makes it a perfect heuristic for A* (admissible and maximally informative).
pub fn longest_increasing_subsequence_length(l: &[i32]) -> usize {
    if l.is_empty() {
        return 0;
    }

    // tails[i] holds the smallest tail element for an increasing subsequence of length i+1
    let mut tails: Vec<i32> = Vec::new();

    for &x in l {
        match tails.binary_search(&x) {
            Ok(_) => {} // x already exists, skip (we need strictly increasing for unique elements)
            Err(pos) => {
                if pos == tails.len() {
                    tails.push(x);
                } else {
                    tails[pos] = x;
                }
            }
        }
    }

    tails.len()
}

pub fn score_by_lis(l: &[i32]) -> usize {
    l.len() - longest_increasing_subsequence_length(l)
}

// Compute the true optimal cost using the `lis` heuristic
pub fn score_by_num_modifications_fast(l: im::Vector<i32>) -> Option<usize> {
    score_by_num_modifications_with_heuristic(l, score_by_lis)
}
