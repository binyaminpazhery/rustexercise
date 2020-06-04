pub fn square_of_sum(n: u32) -> u32 {
let mut sqsum = 0;
    for i in 1..=n{
        sqsum += i.pow(2);
    }
    return sqsum;
}

pub fn sum_of_squares(n: u32) -> u32 {
let mut sqsum = 0;
    for i in 1..=n{
        sqsum += (i*i);
    }
        return sqsum;
}

pub fn difference(n: u32) -> u32 {
        return square_of_sum(n) - sum_of_squares(n);

}
