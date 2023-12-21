use std::ops::Range;

use ndarray::Array;
use ndarray::prelude::*;

use aoc2022::*;

fn main() {
    let arr = read_as_ndarray_num("input/day8/input.txt").expect("Failed to read input");
    let mut visible: Array2<i32> = Array::zeros(arr.raw_dim());

    for y in 0..arr.nrows() {
        // Visibility for rows ->
        let mut min_height = 0;
        for x in 0..arr.ncols() {
            if x == 0 || arr[[y, x]] > min_height {
                visible[[y, x]] = 1;
                min_height = arr[[y, x]];
            }
        }
        drop(min_height);

        // Visibility for rows <-
        let mut min_height = 0;
        for x in (0..arr.ncols()).rev() {
            if x == arr.ncols() - 1 || arr[[y, x]] > min_height {
                visible[[y, x]] = 1;
                min_height = arr[[y, x]];
            }
        }
        drop(min_height);
    }

    for x in 0..arr.ncols() {
        // Visibility for cols down
        let mut min_height = 0;
        for y in 0..arr.nrows() {
            if y == 0 || arr[[y, x]] > min_height {
                visible[[y, x]] = 1;
                min_height = arr[[y, x]];
            }
        }
        drop(min_height);

        // Visibility for cols up
        let mut min_height = 0;
        for y in (0..arr.nrows()).rev() {
            if y == arr.nrows() - 1 || arr[[y, x]] > min_height {
                visible[[y, x]] = 1;
                min_height = arr[[y, x]];
            }
        }
        drop(min_height);
    }

    println!("{}", &arr);
    println!("Visible trees = {}", visible.sum());
    println!("Scenic 2,1 = {}", scenic_score(arr.view(), 2, 1));
    println!("Scenic 2,3 = {}", scenic_score(arr.view(), 2, 3));

    let mut max_score = 0;
    for y in 0..arr.nrows() {
        for x in 0..arr.ncols() {
            let score = scenic_score(arr.view(), x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!("Max scenic score = {}", max_score);
}

fn scenic_score(arr: ArrayView2<u8>, x: usize, y: usize) -> i32 {
    let moves: [[i32; 2]; 4] = [[0, -1], [-1, 0], [1, 0], [0, 1]];
    let boundsx = 0..(arr.ncols() as i32);
    let boundsy = 0..(arr.nrows() as i32);

    let origin_height = arr[[y, x]];
    let mut move_scores: Vec<i32> = vec![];
    // println!("Score for {x},{y}");
    for step in &moves {
        // println!("Step {:?}", &step);
        let mut pos: [i32; 2] = [y as i32, x as i32];
        let mut move_score = 0;
        loop {
            pos[0] += step[0];
            pos[1] += step[1];
            if boundsy.contains(&pos[0]) && boundsx.contains(&pos[1]) {
                // println!("Sove {:?} at {:?} can see", &step, &pos);
                move_score += 1;
                if arr[[pos[0] as usize, pos[1] as usize]] >= origin_height {
                    break;
                }
            } else {
                break;
            }
        }
        move_scores.push(move_score);
    }
    let mut total_score = 1;
    move_scores.iter().for_each(|s| total_score *= s);
    println!("Score at {x},{y} = {move_scores:?} = {total_score}");
    total_score
}
