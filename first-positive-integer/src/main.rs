fn first_missing_positive(mut numbers: Vec<i32> ) -> i32 {
    if numbers.is_empty() { return 1 }

    let mut one_not_found : bool = true;

    // Loop through vector, removing negative numbers and checking if one not found.
    for index in 0..numbers.len() {
        if numbers[index] < 1 {
            numbers[index] = 1;
        }
        else if numbers[index] == 1 {
            one_not_found = false;
        }
    }
    if one_not_found {
        return 1
    }

    /* First missing positive integer must be in [1, numbers.len()].
    For each value in the vector if the value falls in [1, numbers.len()],
    access the value'th entry in the array and set its value to negative its
    current value. This marks us having seen the number index-1 (0-based).
    */
    println!("{:?}", numbers);
    return 0
}

fn main() {
    let my_vector = vec!(1, 4, 2, 6, 4, -7);
    println!("The first missing positive integer in the vector is {}.", first_missing_positive(my_vector));
}