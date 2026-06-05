# ternary-minority

**The minority rule: the cellular automaton that refuses to settle.**

Most cellular automata converge. Game of Life finds still lifes. Majority rule converges to consensus. Even Langton's ant settles into a highway.

The minority rule doesn't converge. Ever.

The rule is simple: each cell looks at itself and its two neighbors (a 3-cell neighborhood on a ring). It takes the value that is *least common* among the three. If all three are the same, the cell flips to the smallest value. The result is **eternal oscillation** — 62.7% of cells are still flipping at tick 300. The system never finds equilibrium because every equilibrium is immediately unstable: if everyone agrees, everyone disagrees.

This is the mathematical basis of *adversarial resilience*. A system governed by the minority rule cannot be captured by any single state — monoculture is dynamically impossible.

## What's Inside

- **`minority_step(grid, width)`** — one tick of the minority rule on a 1D ring
- **`run_minority(grid, width, ticks)`** — full simulation, returns `MinorityResult`
- **`MinorityResult`** — `final_zero_frac`, `oscillating_frac`, `cluster_count`, `energy`
- **`find_oscillators(grid, width)`** — indices of cells that flip every tick
- **`domain_walls(grid, width)`** — count boundaries between +1 and -1 regions
- **`is_stable(grid, width)`** — check if the grid has stopped changing (spoiler: it hasn't)

## Quick Example

```rust
use ternary_minority::*;

// Start with a random-ish ternary ring
let mut grid = vec![0, 1, -1, 0, 1, -1, 0, 1];

// Run 300 ticks
let result = run_minority(&mut grid, 8, 300);

println!("Fraction oscillating: {:.1}%", result.oscillating_frac * 100.0);
// ~62.7% — the majority of cells never stop flipping

println!("Zero fraction: {:.2}", result.final_zero_frac);
// The 0 state absorbs minority dynamics

// Find the specific cells that oscillate
let oscillators = find_oscillators(&grid, 8);
println!("{} cells are still flipping", oscillators.len());

// Check stability (spoiler: false)
assert!(!is_stable(&grid, 8));
```

## The Deeper Truth

**The minority rule is the immune system of ternary dynamics.** It prevents any single state from dominating by *definition* — the rule explicitly opposes the majority. This has a precise mathematical consequence: in a 3-state system, the minority rule creates a *frustrated* lattice. Every local neighborhood wants to be diverse, but satisfying all neighborhoods simultaneously is impossible. The frustration propagates forever.

The 62.7% oscillation fraction isn't random — it's the steady-state fraction of cells that are *structurally* frustrated. These cells sit at domain boundaries where the neighborhood never resolves. The remaining ~37% are in "trapped" regions where the local neighborhood happens to be stable, but even these can become unstable if neighboring regions shift.

The domain walls — boundaries between +1 and -1 regions — are the most important structures. They're the active fronts where the minority rule does its work. Walls move, merge, split, and reform, but they never vanish entirely.

**Use cases:**
- **Anti-monoculture design** — the minority rule as a constitutional guard against consensus
- **Adversarial robustness** — systems that resist capture by design
- **Creative generation** — eternal oscillation as a source of novelty
- **Neural network regularization** — minority rule as a different kind of dropout
- **Game theory** — the minority game as a model of financial markets

## See Also

- **ternary-drift** — the opposite: random drift that *causes* monoculture
- **ternary-consensus** — the consensus mechanisms that minority rule prevents
- **ternary-life** — another CA with different dynamics (lifecycle, not minority)
- **ternary-percolation** — how minority-driven patterns percolate through space

## Install

```bash
cargo add ternary-minority
```

## License

MIT
