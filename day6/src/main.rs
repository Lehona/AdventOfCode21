fn main() {
    let input = include_str!("input.txt");
    let numbers: Vec<usize> = input.split(",").map(|n|n.parse().unwrap()).collect();

    part1(&numbers);
    part2(&numbers);
}

fn part1(numbers: &[usize]) {
    let mut buckets = [0; 9];

    for &n in numbers {
    	buckets[n] += 1;
    }

    for _ in 0..80 {
    	let mut next_buckets = [0; 9];
		for i in 1..9 {
			next_buckets[i-1] = buckets[i];
		}
		next_buckets[6] += buckets[0];
		next_buckets[8] += buckets[0];
		buckets = next_buckets;
    }

    println!("After 80 days there are {} lanternfish.", buckets.iter().sum::<usize>());

}

fn part2(numbers: &[usize]) {
    let mut buckets = [0; 9];

    for &n in numbers {
    	buckets[n] += 1;
    }

    for _ in 0..256 {
    	let mut next_buckets = [0; 9];
		for i in 1..9 {
			next_buckets[i-1] = buckets[i];
		}
		next_buckets[6] += buckets[0];
		next_buckets[8] += buckets[0];
		buckets = next_buckets;
    }

    println!("After 256 days there are {} lanternfish.", buckets.iter().sum::<usize>());

}
