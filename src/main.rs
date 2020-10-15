use std::io;
use rand::Rng;

fn show(arr: & [[usize; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            print!("{} ", arr[i][j]);
        }
        println!("");
    }
    println!("############");
}

fn down_item(arr: &mut [[usize; 4]; 4], y: usize, x: usize) {
    let mut yindex: usize = y;
    for i in (y+1)..4 {
        let v = arr[i][x];
        if v==0 || v ==  arr[yindex][x] {
            arr[i][x] += arr[yindex][x];
            arr[yindex][x] = 0;
            yindex = i;
            continue;
        }
    }
}

fn down(arr: &mut [[usize; 4]; 4]) {
    for i in 0..3 {
        let obj_row: usize = 2 - i;
        for j in 0..4 {
            if arr[obj_row][j]>0 {
                down_item(arr, obj_row, j);
            }
        }
    }
}

fn up_item(arr: &mut [[usize; 4]; 4], y: usize, x: usize) {
    let mut yindex: usize = y;
    for i2 in 0..y {
        let i = y-1 - i2;
        let v = arr[i][x];
        if v==0 || v ==  arr[yindex][x] {
            arr[i][x] += arr[yindex][x];
            arr[yindex][x] = 0;
            yindex = i;
            continue;
        }
    }
}

fn up(arr: &mut [[usize; 4]; 4]) {
    for i in 1..4 {
        let obj_row: usize = i;
        for j in 0..4 {
            if arr[obj_row][j]>0 {
                up_item(arr, obj_row, j);
            }
        }
    }
}

fn left(arr: &mut [[usize; 4]; 4]) {
    for i in 1..4 {
        let obj_col: usize = i;
        for j in 0..4 {
            if arr[j][obj_col]>0 {
                left_item(arr, j, obj_col);
            }
        }
    }
}

fn left_item(arr: &mut [[usize; 4]; 4], y: usize, x: usize) {
    let mut xindex: usize = x;
    for i2 in 0..x {
        let i = x-1 - i2;
        let v = arr[y][i];
        if v==0 || v ==  arr[y][xindex] {
            arr[y][i] += arr[y][xindex];
            arr[y][xindex] = 0;
            xindex = i;
            continue;
        }
    }
}

fn right(arr: &mut [[usize; 4]; 4]) {
    for col2 in 0..3 {
        let col = 2 - col2;
        for row in 0..4 {
            if arr[row][col]>0 {
                right_item(arr, row, col);
            }
        }
    }
}

fn right_item(arr: &mut [[usize; 4]; 4], y: usize, x: usize) {
    let mut xindex: usize = x;
    for i in (x+1)..4 {
        let v = arr[y][i];
        if v==0 || v ==  arr[y][xindex] {
            arr[y][i] += arr[y][xindex];
            arr[y][xindex] = 0;
            xindex = i;
            continue;
        }
    }
}

fn seed(arr: &mut [[usize; 4]; 4]) {
    for _ in 0..2 {
        let mut rng = rand::thread_rng();
        let i: u8 = rng.gen();
        let j: u8 = rng.gen();
        let i = (i%4) as usize;
        let j = (j%4) as usize;
        if arr[i][j]==0 {
            arr[i][j] = 2;
        }
    }
}

fn main() {
    println!("{:?}", String::from("23333"));
    let mut arr: [[usize; 4]; 4] = [[0;4];4];
    arr[1][1] = 2;
    arr[2][1] = 2;
    arr[3][1] = 4;
    show(&arr);
    while true {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let num = input.trim().as_bytes()[0];
        match num {
            b'w' =>  up(&mut arr),
            b's' =>  down(&mut arr),
            b'a' =>  left(&mut arr),
            b'd' =>  right(&mut arr),
            _ => (),
        }
        seed(&mut arr);
        show(&arr);
    }
}