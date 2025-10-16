fn placeQueen(pos: (usize, usize), placement: &mut Vec<Vec<u8>>) {
    let mut dis = pos.0 + pos.1;
    let N = placement.len();
    let mut disx = 2 * N - pos.1 + pos.0;
    for j in 0..N {
        placement[pos.0][j] += 1;
        placement[j][pos.1] += 1;

        if dis >= 0 && dis < placement.len() {
            placement[dis][j] += 1;
            print_placments(&placement);
        }
        if disx >= 0 && disx < placement.len() {
            //placement[disx][j] += 1;
            print_placments(&placement);
        }
        if dis != 0 {
            dis -= 1;
        }
        if disx != 0 {
            disx -=1;
        }
    }
    placement[pos.0][pos.1] = b'Q';
}
fn removeQueen(pos: (usize, usize), placement: &mut Vec<Vec<u8>>) {
    let mut dis = pos.0 + pos.1;
    let mut disx = pos.0 + pos.1;
    let N = placement.len();
    for j in 0..N {
        placement[pos.0][j] -= 1;
        placement[j][pos.1] -= 1;

        if dis >= 0 && dis < placement.len() {
            print_placments(&placement);
            placement[dis][j] -= 1;
            print_placments(&placement);
            if dis > 0 {
                placement[N - j - 1][dis] -= 1;
            }
        }
        if dis != 0 {
            dis -= 1;
        }
    }
    placement[pos.0][pos.1] = b'.';
}
fn print_placments(placements: &Vec<Vec<u8>>) {
    println!("-----------------------");
    for idx in 0..placements.len() {
        println!(" {:?} ", placements[idx]);
    }
    println!("-----------------------");
}

fn main() {
    println!("Hello, world!");
    let mut placement = vec![vec![b'.'; 5]; 5];
    placeQueen((1, 2), &mut placement);
    print_placments(&placement);
    //removeQueen((2, 1), &mut placement); print_placments(&placement);
}
