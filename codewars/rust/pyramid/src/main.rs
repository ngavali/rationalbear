fn pyramid(n: usize) -> Vec<Vec<u32>> {
    let mut result = Vec::with_capacity(n);
    let mut i = 0;
    while i < n {
        i += 1;
        result.push(vec![1; i]);
    }

    result
}

fn main() {
    /*
    let mut array = [1, 2, 3];
    let sl1 = &array[0..2];
    let sl2 = &array;

    array[1] = 20;
    println!("{:?} {:?}", sl1, sl2);
    println!("{:?}", pyramid(3));
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(pyramid(0), vec![] as Vec<Vec<u32>>);
        assert_eq!(pyramid(1), vec![vec![1]]);
        assert_eq!(pyramid(2), vec![vec![1], vec![1, 1]]);
        assert_eq!(pyramid(3), vec![vec![1], vec![1, 1], vec![1, 1, 1]]);
    }
}
