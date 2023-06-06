fn bin_search_vec(vector: Vec<i32>, number: i32) -> bool {
    // assumes the vector parameter is sorted low to high
    let mut low = 0;
    let mut high = vector.len();

    while low < high {
        // pointer to the middle of the vector
        let mid = low + (high - low) / 2;
        let val = vector[mid];

        if val == number {
            return true;
        } else if val > number {
            // if current val is higher than the searched number
            // drop the right side of the list 
            high = mid;
        } else {
            // else drop the left side of the left
            low = mid + 1;
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
            assert_eq!(bin_search_vec(v, 1), true);
    }
    #[test]
        fn test_search_fail() {
            let v = vec![1,2,3,4,5];
            assert_eq!(bin_search_vec(v, 1000), false);
    }

}
