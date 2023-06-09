fn inplace_bubble_sort(coll : &mut Vec<i32>) {
    let len = coll.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if coll[j] > coll[j+1] {
                let tmp = coll[j];
                coll[j] = coll[j+1];
                coll[j+1] = tmp;
            }
        }
    }
}

fn main() {
    let mut coll : Vec<i32> = vec![2,3,4,-1, 9, 2];
    println!("{:?}", inplace_bubble_sort(&mut coll));
}


#[cfg(test)]
mod tests {
    use super::*;

//    #[test]
//        fn test_search_success() {
//            let v = vec![1,2,3,4,5];
//            assert_eq!(bin_search_vec(v, 1), true);
//    }
//    #[test]
//        fn test_search_fail() {
//            let v = vec![1,2,3,4,5];
//            assert_eq!(bin_search_vec(v, 1000), false);
//    v

}
