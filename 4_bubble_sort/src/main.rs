fn inplace_bubble_sort(coll : &mut Vec<i32>) {
    let len = coll.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if coll[j] > coll[j+1] {
                let tmp = coll[j];
                coll[j] = coll[j+1];
                coll[j+1] = tmp;

            }
        }
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
        fn test_bubble_sort() {
            let mut coll : Vec<i32> = vec![2,3,4,-1,9,2];
            inplace_bubble_sort(&mut coll); 
            assert_eq!(coll, vec![-1, 2, 2, 3, 4, 9]);
    }
}
