fn search_vec(vector : Vec<i32>, number: i32) -> bool {
   for i in 0..vector.len() {
        let item = vector[i];
        if item == number {
            return true;
        }
    }
    false
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
        fn test_search_success() {
            let v = vec![1,2,3,4,5];
            assert_eq!(search_vec(v, 1), true);
    }
    #[test]
        fn test_search_fail() {
            let v = vec![1,2,3,4,5];
            assert_eq!(search_vec(v, 1000), false);
    }

}
