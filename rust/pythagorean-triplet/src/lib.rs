use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set: HashSet<[u32; 3]> = HashSet::new();
    for a in 2..sum {
	for b in a..(sum - a) {
	    let c: u32 = sum - a - b;
	    if a.pow(2) + b.pow(2) == c.pow(2) {
		set.insert([a, b, c]);
	    }
	}
    }
    return set;
}
