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
fn score_by_exact_placement(l: &int[]) {
    unimplemented!() 
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
fn score_by_distance(l: &int[]) {
    unimplemented!()
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
fn score_by_relative_placement(l: &int[]) {
    unimplemented!()
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
fn score_by_correct_neighbors(l: &int[]) {
    unimplemented!()
}

// implementation 4 - score by num modifications:
// This is a heuristic that would be easy for a human to understand, but could be hard for them to
// implement.  Imagine that you have a small hand of playing cards laid out on a table in some
// order.  We can define "how sorted are these cards" as "what is the smallest number of
// modifications to this list are necessary to get it into sorted order", where a "modification" is
// defined as picking up any card and putting it anywhere else in the list.
//
// I don't know if there's a good algorithm for computing this, so I suggest searching through the
// space of modifications using A* search.  Here's some OCaml pseudo-code:
//
// ```ocaml
//
// let heuristic l = score_by_exact_placement l 
//
// let is_done l = score_by_exact_placement l = 0 
//
// let neighbors l = 
//   let indexes = List.mapi l ~f:(fun i _ -> i) in
//   List.concat_map indexes ~f:(fun k -> 
//     let without, removed_value = List.remove_at l k in 
//     List.map indexes ~f:(fun k -> 
//       List.insert_at l k removed_value))
// ;;
//  
// let score_by_num_modifications l = 
//   let path = astar ~heuristic ~is_done ~neighbors l in
//   List.length path
// ;;
// ```
//
// A score of 0 is perfect, and larger scores are worse
fn score_by_num_modifications(l: im::Vec<int>) {
    unimplemented!()
}

// TODO: longest sorted run

// TODO: longest correct sequence (no gaps)

// TODO: longest correct sequence (no gaps)
