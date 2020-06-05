pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
    	println!("Incorrect Input");
    }
    let mut value=1;
    for _ in 1..s {
    	value=value*2;
    }
    return value;
}

pub fn total() -> u64 {
    let mut sum=0;
    for i in 1..65{
    	sum=sum+square(i)
    }
    return sum;
}
