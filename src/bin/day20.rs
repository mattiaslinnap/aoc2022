#![allow(dead_code, unused_variables)]

use aoc2022::*;

#[derive(Debug)]
struct Val {
    val: i32,
    moved: bool,
}

fn main() {
    let ints = read_lines_as_ints("input/day20/test.txt").expect("Failed to read input");
    let vals: Vec<Val> = ints.iter().map(|&x| Val { val: x, moved: false }).collect();
    // let ns = vals.len();
    // let ni = ns as i32;

    println!("Initial:");
    print_state(&vals);

    // loop {
    //     let maybe_first_unmoved_idx = vals.iter().position(|v| !v.moved);
    //     match maybe_first_unmoved_idx {
    //         Some(idx) => {
    //             let there = vals.remove(idx).val;
    //             let idx = idx as i32;
    //             let base_new_idx = (idx + there + (ni * 3)) % ni;
    //
    //             let mut offset = 0i32;
    //             if base_new_idx >= idx {
    //                 offset = 0;
    //             } else if base_new_idx < idx {
    //                 offset = -1;
    //             } else {
    //                 offset = -1;
    //             }
    //
    //             let new_idx = ((idx + there + (ni * 3) + offset) % ni) as usize;
    //             println!();
    //             println!("{there} moves to {new_idx}: orig={idx} + num={there} + off={offset} = new={new_idx}");
    //             vals.insert(new_idx, Val { val: there, moved: true });
    //
    //             print_state(&vals);
    //         }
    //         None => {
    //             break;
    //         }
    //     }
    // }
    //
    // let nums: Vec<i32> = vals.iter().map(|v| v.val).collect();
    // let zero_idx = nums.iter().position(|&x| x == 0).expect("Zero not found");
    //
    // let k1 = nums[(zero_idx + 1000) % ns];
    // let k2 = nums[(zero_idx + 2000) % ns];
    // let k3 = nums[(zero_idx + 3000) % ns];
    // let k = k1 + k2 + k3;
    // println!("{} + {} + {} = {}", k1, k2, k3, k);
}
//
// fn move_idx<T>(vals: Vec<T>, idx: usize, by: usize) {}
//
// fn move_val(vals: Vec<&Val>, idx: usize) {
//     move_idx(vals, idx, wrap_pos(vals[idx].val, vals.len()));
// }
//
// fn move_int(vals: Vec<i32>, idx: usize) {
//     move_idx(vals, idx, wrap_pos(vals[idx], vals.len()));
// }
//
// fn wrap_pos(idx: i32, len: usize) -> usize {
//     idx.rem_euclid(len as i32) as usize
// }

fn print_state(vals: &Vec<Val>) {
    let out: Vec<i32> = vals.iter().map(|x| x.val).collect();
    println!("{:?}", &out[0..]);
}
