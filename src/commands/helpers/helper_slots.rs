use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use rand::rng;

const LINES: [[usize; 5]; 10] = [
    [1, 1, 1, 1, 1],  // Line 0
    [0, 0, 0, 0, 0],  // Line 1
    [2, 2, 2, 2, 2],  // Line 2
    [0, 1, 2, 1, 0],  // Line 3
    [2, 1, 0, 1, 2],  // Line 4
    [1, 2, 2, 2, 1],  // Line 5
    [1, 0, 0, 0, 1],  // Line 6
    [2, 2, 1, 0, 0],  // Line 7
    [0, 0, 1, 2, 2],  // Line 8
    [2, 1, 1, 1, 0],  // Line 9
];

const PAYOUTS: [[usize; 3]; 10] = [
    [100, 1000, 5000],
    [40, 400, 2000],
    [18, 180, 1800],
    [30, 100, 750],
    [30, 100, 750],
    [5, 40, 150],
    [5, 40, 150],
    [5, 25, 100],
    [5, 25, 100],
    [5, 25, 100],
];

//TO-DO: read from file
const DISTRIBUTION: [f64; 10] = [
    0.007,  // Prob for 0
    0.01,  // Prob for 1
    0.008,  // Prob for 2
    0.0125,  // Prob for 3
    0.0125,  // Prob for 4
    0.085,  // Prob for 5
    0.085,  // Prob for 6
    0.26,  // Prob for 7
    0.26,  // Prob for 8
    0.26  // Prob for 9
];

pub fn generate_grid(rows: usize, cols: usize) -> Vec<Vec<u8>> {
    (0..rows)
        .map(|_| {
            (0..cols)
                .map(|_| get_random_weighted_symbol())
                .collect()
        })
        .collect()
}

pub fn calculate_winnings(grid: &Vec<Vec<u8>>, lines_to_check: u32, bet: u32) -> usize {
    let matches = check_consecutive_matches(grid, &LINES[..lines_to_check as usize], 2, 3);
    let mut total = 0;

    for (streak, symbol) in matches {
        if let Some(payouts) = PAYOUTS.get(symbol as usize) {
            if let Some(payout_coeff) = payouts.get(streak - 3) {
                total += payout_coeff * bet as usize;
            }
        }
    }

    total
}

fn check_consecutive_matches(
    grid: &Vec<Vec<u8>>,
    lines: &[[usize; 5]],
    wildcard: u8,
    min_consecutive: usize,
) -> Vec<(usize, u8)> {
    let mut results = Vec::new();

    for line in lines {
        let mut values: Vec<u8> = Vec::with_capacity(5);
        for (col, &row) in line.iter().enumerate() {
            values.push(grid[row][col]);
        }

        let mut base_val: Option<u8> = None;
        let mut streak = 0;
        let mut prefix_wilds = 0;

        for &val in &values {
            if base_val.is_none() {
                if val == wildcard {
                    prefix_wilds += 1;
                } else {
                    base_val = Some(val);
                    streak = 1 + prefix_wilds;
                }
            } else {
                if Some(val) == base_val || val == wildcard {
                    streak += 1;
                } else {
                    break;
                }
            }
        }

        if base_val.is_none() && prefix_wilds >= min_consecutive {
            base_val = Some(wildcard);
            streak = prefix_wilds;
        }

        if let Some(val) = base_val {
            if streak >= min_consecutive {
                results.push((streak, val));
            }
        }
    }

    results
}

fn get_random_weighted_symbol() -> u8 {
    let dist = WeightedIndex::new(&DISTRIBUTION).unwrap();
    let mut rng = rng();
    dist.sample(&mut rng) as u8
}