use rand::Rng;

fn where_does_crystal_ball_break(breaks: Vec<bool>) -> i32 {
   // Two Crystal Ball problem: 
   // Given two crystal balls that will break if dropped from high enough distance, 
   // determine the exact spot in which it will break in the most optimized way
    // breaks vector represents the fall of a ball.
    // The vector is expected in the form of [false, false, false, true true]
    // solution in O(sqrt(N)) time 

    // determine how far ahead to jump in the vector
    let jump_distance = (breaks.len() as f32)
                                         .sqrt()
                                         .floor() as usize;
    // the first jump   
    let mut i = jump_distance; 
    
    // look for occurence of true value in the breaks vector
    while i < breaks.len()  {
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

struct Breaks {
    breaks: Vec<bool>,
    break_point: u32,
}

fn get_breaks(max_floors: u32) -> Breaks {
    let mut rng = rand::thread_rng();
    // how many floors are in the building? there has to be at least 1
    let floors_amount = rng.gen_range(1..max_floors);
    // at which floor does the crystal ball break?
    let break_point = rng.gen_range(0..floors_amount);
    let breaks = (0..floors_amount).map(|v| { if v < break_point { false } else { true } }).collect();
    println!("{:?}", floors_amount); 
    println!("{:?}", break_point); 
    println!("{:?}", breaks);
    Breaks {
        breaks: breaks,
        break_point: break_point,
    }
}

fn main() {
    // let breaks : Breaks = get_breaks(10);
    // println!("{:?}", where_does_crystal_ball_break(breaks.breaks));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
        fn test_crystal_ball_break() {
            let breaks : Breaks = get_breaks(10);
            assert_eq!(where_does_crystal_ball_break(breaks.breaks), breaks.break_point as i32);
    }
   // #[test]
   //     fn test_crystal_ball_doesnt_break() {
   //         let breaks = vec![false, false, false, false];
   //         assert_eq!(where_does_crystal_ball_break(breaks), -1);
   // }   
    
}
