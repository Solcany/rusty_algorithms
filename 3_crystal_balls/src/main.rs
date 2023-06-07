fn where_does_crystal_ball_break(breaks: Vec<bool>) -> i32 {
   // Two Crystal Ball problem: 
   // Given two crystal balls that will break if dropped from high enough distance, 
   // determine the exact spot in which it will break in the most optimized way
    // breaks vector represents the fall of a ball.
    // The vector is expected in the form of [false, false, false, true true]
    // solution in O(sqrt(N)) time 

    // determine how far ahead to jump in the array
    let jump_distance = (breaks.len() as f32)
                                         .sqrt()
                                         .floor() as usize;
    // the first jump   
    let mut i = jump_distance; 
    
    // look for occurence of true value in the breaks vector
    while i < breaks.len() {
        // stop when the first occurence is found 
        if breaks[i] {
            break;
        }
        // jump ahead
        i += jump_distance;
    }
    // it's likely that a true value was skipped when jumping ahead
    // therefore walk back the jumping distance
    i -= jump_distance;
    
    // check jump_distance amount of values preceding the first found occurence of truth
    let mut j : usize = 0;

    while j < jump_distance && i < breaks.len() {
        if breaks[i] {
            // return the very first occurence of truth
            return i as i32;
        }
        j += 1;
        i += 1;
    }

    -1
}

fn main() {
    
    let breaks = vec![false, false, false, false, true, true, true, true, true, true];
    println!("{}", where_does_crystal_ball_break(breaks));          
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
        fn test_crystal_ball_break() {
            let breaks = vec![false, false, false, false, true, true, true];
            assert_eq!(where_does_crystal_ball_break(breaks), 4);
    }
    #[test]
        fn test_crystal_ball_doesnt_break() {
            let breaks = vec![false, false, false, false];
            assert_eq!(where_does_crystal_ball_break(breaks), -1);
    }   
    
}
