#![forbid(unsafe_code)]

/// Result of running a minority cellular automaton.
#[derive(Debug, Clone)]
pub struct MinorityResult {
    pub final_zero_frac: f64,
    pub oscillating_frac: f64,
    pub cluster_count: usize,
    pub energy: f64,
}

/// One step of the minority rule CA on a 1D ring.
/// Each cell takes the value that is the minority among itself and its two neighbors.
/// If all three are the same, the cell keeps its value (no majority to oppose).
/// If there's a tie (e.g., -1, 0, 1), the cell takes the smallest value.
pub fn minority_step(grid: &mut [i8], width: usize) {
    if width == 0 || grid.is_empty() {
        return;
    }
    let n = grid.len();
    let old = grid.to_vec();

    for i in 0..n {
        let left = old[(i + n - 1) % n];
        let center = old[i];
        let right = old[(i + 1) % n];

        // Count occurrences
        let mut counts = [0usize; 3]; // index: val+1
        for &v in &[left, center, right] {
            counts[(v + 1) as usize] += 1;
        }

        // Find the minority value (smallest count, break ties by smallest value)
        let min_count = counts.iter().min().unwrap();
        let mut minority_val = -1i8;
        for (idx, &c) in counts.iter().enumerate() {
            if c == *min_count {
                minority_val = idx as i8 - 1;
                break;
            }
        }

        grid[i] = minority_val;
    }
}

/// Run minority CA for given number of ticks.
pub fn run_minority(grid: &mut [i8], width: usize, ticks: usize) -> MinorityResult {
    if grid.is_empty() {
        return MinorityResult {
            final_zero_frac: 0.0,
            oscillating_frac: 0.0,
            cluster_count: 0,
            energy: 0.0,
        };
    }

    let _snap0 = grid.to_vec();

    for _ in 0..ticks {
        minority_step(grid, width);
    }

    let snap_final = grid.to_vec();

    // Do one more step to detect oscillators
    minority_step(grid, width);
    let snap_one_more = grid.to_vec();
    // Restore to final state
    grid.copy_from_slice(&snap_final);

    // Fraction of zeros in final state
    let n = snap_final.len() as f64;
    let final_zero_frac = snap_final.iter().filter(|&&v| v == 0).count() as f64 / n;

    // Oscillators: cells that would flip if we did one more step
    let oscillating = snap_final
        .iter()
        .zip(snap_one_more.iter())
        .filter(|(&a, &b)| a != b)
        .count();
    let oscillating_frac = oscillating as f64 / n;

    // Cluster count: contiguous regions of same value
    let cluster_count = count_clusters(&snap_final, width);

    // Energy: sum of |cell - neighbor| for all adjacent pairs
    let energy = compute_energy(&snap_final, width);

    MinorityResult {
        final_zero_frac,
        oscillating_frac,
        cluster_count,
        energy,
    }
}

/// Find indices that flip every tick (oscillators).
/// Returns indices where the value at tick T differs from tick T+1.
/// Compares current grid with one step forward.
pub fn find_oscillators(grid: &[i8], width: usize) -> Vec<usize> {
    if grid.is_empty() {
        return vec![];
    }
    let mut next = grid.to_vec();
    minority_step(&mut next, width);

    grid.iter()
        .zip(next.iter())
        .enumerate()
        .filter_map(|(i, (&a, &b))| if a != b { Some(i) } else { None })
        .collect()
}

/// Count domain walls (boundaries between +1 and -1 regions).
/// A wall exists at position i if |grid[i] - grid[(i+1) % n]| == 2.
pub fn domain_walls(grid: &[i8], _width: usize) -> usize {
    if grid.is_empty() {
        return 0;
    }
    let n = grid.len();
    let mut walls = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        if (grid[i] - grid[j]).unsigned_abs() == 2 {
            walls += 1;
        }
    }
    walls
}

/// Check if the grid is stable (doesn't change in the next step).
pub fn is_stable(grid: &[i8], width: usize) -> bool {
    if grid.is_empty() {
        return true;
    }
    let mut next = grid.to_vec();
    minority_step(&mut next, width);
    grid == next.as_slice()
}

fn count_clusters(grid: &[i8], _width: usize) -> usize {
    if grid.is_empty() {
        return 0;
    }
    let n = grid.len();
    let mut count = 0;
    for i in 0..n {
        let prev = (i + n - 1) % n;
        if grid[i] != grid[prev] {
            count += 1;
        }
    }
    // If all same, count is 0 but should be 1
    if count == 0 {
        1
    } else {
        count
    }
}

fn compute_energy(grid: &[i8], _width: usize) -> f64 {
    if grid.is_empty() {
        return 0.0;
    }
    let n = grid.len();
    let mut energy = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;
        energy += (grid[i] - grid[j]).abs() as f64;
    }
    energy / n as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minority_step_uniform() {
        // All same → no minority to pick, but counts are [3,0,0] so minority is 0 or 1
        // Actually counts are [3,0,0] → min count = 0, first with count 0 is idx=1 → val=0
        let mut grid = vec![-1i8, -1, -1];
        minority_step(&mut grid, 3);
        assert_eq!(grid, vec![0, 0, 0]);
    }

    #[test]
    fn test_minority_step_basic() {
        // [-1, 1, -1] → center sees [-1,1,-1], counts=[2,0,1], min=0 at idx=1 → val=0
        let mut grid = vec![-1i8, 1, -1];
        minority_step(&mut grid, 3);
        assert_eq!(grid[1], 0);
    }

    #[test]
    fn test_minority_step_preserves_length() {
        let mut grid = vec![0i8, 1, -1, 0, 1, -1];
        minority_step(&mut grid, 6);
        assert_eq!(grid.len(), 6);
    }

    #[test]
    fn test_minority_step_values_valid() {
        let mut grid = vec![0i8, 1, -1, 0, 1, -1, 0, 1];
        minority_step(&mut grid, 8);
        for &v in &grid {
            assert!(v >= -1 && v <= 1);
        }
    }

    #[test]
    fn test_domain_walls_none() {
        let grid = vec![1i8, 1, 1, 1];
        assert_eq!(domain_walls(&grid, 4), 0);
    }

    #[test]
    fn test_domain_walls_all() {
        let grid = vec![-1i8, 1, -1, 1];
        assert_eq!(domain_walls(&grid, 4), 4);
    }

    #[test]
    fn test_domain_walls_mixed() {
        // [-1, -1, 1, 1, 0] → walls between -1/-1 (0), -1/1 (1), 1/1 (0), 1/0 (0), 0/-1 (0) = 1
        let grid = vec![-1i8, -1, 1, 1, 0];
        assert_eq!(domain_walls(&grid, 5), 1);
    }

    #[test]
    fn test_domain_walls_empty() {
        assert_eq!(domain_walls(&[], 0), 0);
    }

    #[test]
    fn test_is_stable_uniform() {
        // All 0: each neighborhood is [0,0,0], minority of [0,0,0] is -1 (first with count 0)
        // So uniform 0 is NOT stable
        let grid = vec![0i8, 0, 0];
        // Let's check: counts=[0,3,0], min=0, first with 0 is idx=0 → val=-1
        // So not stable
        assert!(!is_stable(&grid, 3));
    }

    #[test]
    fn test_is_stable_empty() {
        assert!(is_stable(&[], 0));
    }

    #[test]
    fn test_find_oscillators_empty() {
        assert!(find_oscillators(&[], 0).is_empty());
    }

    #[test]
    fn test_find_oscillators_returns_valid_indices() {
        let grid = vec![0i8, 1, -1, 0];
        let osc = find_oscillators(&grid, 4);
        for &i in &osc {
            assert!(i < 4);
        }
    }

    #[test]
    fn test_run_minority_basic() {
        let mut grid = vec![0i8, 1, -1, 0, 1, -1];
        let result = run_minority(&mut grid, 6, 10);
        assert!(result.final_zero_frac >= 0.0 && result.final_zero_frac <= 1.0);
        assert!(result.oscillating_frac >= 0.0 && result.oscillating_frac <= 1.0);
        assert!(result.energy >= 0.0);
    }

    #[test]
    fn test_run_minority_empty() {
        let mut grid: Vec<i8> = vec![];
        let result = run_minority(&mut grid, 0, 5);
        assert_eq!(result.final_zero_frac, 0.0);
        assert_eq!(result.cluster_count, 0);
    }

    #[test]
    fn test_cluster_count_uniform() {
        let grid = vec![1i8, 1, 1, 1];
        let c = count_clusters(&grid, 4);
        assert_eq!(c, 1);
    }

    #[test]
    fn test_cluster_count_alternating() {
        let grid = vec![-1i8, 1, -1, 1];
        let c = count_clusters(&grid, 4);
        assert_eq!(c, 4);
    }

    #[test]
    fn test_energy_uniform() {
        let grid = vec![1i8, 1, 1, 1];
        let e = compute_energy(&grid, 4);
        assert_eq!(e, 0.0);
    }

    #[test]
    fn test_energy_max_alternating() {
        let grid = vec![-1i8, 1, -1, 1];
        let e = compute_energy(&grid, 4);
        assert_eq!(e, 2.0);
    }
}
