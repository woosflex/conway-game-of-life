/*
PSEUDOCODE:
1. Take input from the user for the height and width of the grid.
2. Check for the validity of the input.
3. If the input is invalid, print an error message and terminate the program.
4. If the input is valid, create a 2d Array of size height * width.
5. Initialize the array with empty cells.
6. Take input from the user for the initial number of live cells.
7. Check for the number of live cells to be less than the total number of cells in the grid or a negative number or zero.
8. If the number of live cells is greater than the total number of cells in the grid or a negative number or zero, print an error message and terminate the program.
9. If the number of live cells is less than the total number of cells in the grid, randomly populate the array with live cells.
10. Print the initial state of the grid.
11. Store the initial state of the grid in another array for the generation of the next grid.
12. Apply the rules of the Game of Life to the grid and generate the next grid w.r.t the boundaries of the grid.
12.1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
12.2. Any live cell with two or three live neighbours lives on to the next generation.
12.3. Any live cell with more than three live neighbours dies, as if by overpopulation.
12.4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
13. Repeat steps 10-12 where initial grid becomes n-1, where n is the current state of grid, and two copies atlernate with each other for the generation of the next grid.
14. Keep until the user terminates the program.

DATA STRUCTURES:
1. Two 2d arrays of size height * width with boolean values to represent the state of the cell. (0 for empty/dead, 1 for live)
*/

use std::io;
use std::{thread, time};

fn take_input() -> u32 {
    let mut input: u32 = 0;
    let mut valid_input: bool = false;
    let mut input_text = String::new();
    while valid_input != true {
        input_text.clear();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read line");
        match input_text.trim().parse::<u32>() {
            Ok(num) => {
                input = num;
                valid_input = true;
            }
            Err(_) => {
                println!("Please type a number!");
            }
        };
    };
    return input;
}

fn print_grid(grid: &Vec<Vec<bool>>, height: u32, width: u32) {
    print!("{}[2J", 27 as char);
    for row in 0..height as usize {
        for col in 0..width as usize {
            match grid[row][col] {
            true => print!("\x1b[32m0\x1b[0m"),
            false => print!("\x1b[31mO\x1b[0m")
            }
        }
        println!();
    }
}

fn update_grid(grid: &mut Vec<Vec<bool>>, height: u32, width: u32) {
    let mut buffer: Vec<Vec<bool>> = grid.clone();

    for row in 0..height {
        for col in 0..width {
            let mut count: u32 = 0;
            for i in (row as i32 - 1)..(row as i32 + 2) {
                for j in (col as i32 - 1)..(col as i32 + 2) {
                    if !(i < 0) && !(i >= height as i32) && !(j < 0) && !(j >= width as i32) {
                        if i == row as i32 && j == col as i32 {
                            continue;
                        }
                        if grid[i as usize][j as usize] == true {
                            count += 1;
                        }
                    }
                }
            }
            if count < 4 && count >= 2 {
                if count == 2 && grid[row as usize][col as usize] == false {
                    buffer[row as usize][col as usize] = false;
                } else {
                    buffer[row as usize][col as usize] = true;
                }
            } else {
                buffer[row as usize][col as usize] = false;
            }
        }
    }
    *grid = buffer;
}

fn main() {
    println!("Enter the height of the grid: ");
    let height: u32 = take_input();
    println!("Enter the width of the grid: ");
    let width: u32 = take_input();

    println!("Height: {}", height);
    println!("Width: {}", width);

    let mut grid: Vec<Vec<bool>> = vec![vec![false; width as usize]; height as usize];

    println!("Enter the number of alive cells in the start: ");

    let alive_cells: u32 = take_input();
    if alive_cells >= (height * width) {
        println!("Number of alive cells can't be more than number of cells in grid.");
        return;
    }

    let mut count: u32 = 0;
    loop {
        let col: usize = rand::random_range(0..width as usize);
        let row: usize = rand::random_range(0..height as usize);
        if grid[row][col] != true {
            grid[row][col] = true;
            count += 1;
        }
        if count == alive_cells {
            break;
        }
    }

    println!("Grid {:?}", grid);

    let th_millis = time::Duration::from_millis(17);
    let now = time::Instant::now();

    loop {
        print_grid(&grid, height, width);
        update_grid(&mut grid, height, width);
        thread::sleep(th_millis);
        assert!(now.elapsed() >= th_millis);
    }
}
