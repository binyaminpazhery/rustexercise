pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut result = vec![];

    for i in 0..factors.len() {
    	let mut multiples = vec![];
    	let factor = factors[i];
 
    	for j in factor..limit {
    		if j % factor == 0 {
    			multiples.push(j);
    		}
 
    	}
    	result.append(&mut multiples);

    }

    result.sort();
    result.dedup();
    return result.iter().sum();
}
