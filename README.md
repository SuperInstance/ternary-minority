# ternary-minority

Minority-rule cellular automaton on a 1D ternary ring {-1, 0, +1}. Each cell adopts the *minority* value among itself and its two neighbors, producing self-organizing patterns, oscillators, and domain walls.

## Why It Matters

The minority rule is the inverse of majority-rule voting dynamics. Instead of conforming to neighbors, cells oppose the local majority — a model of anti-conformism, diversification, or competitive exclusion. In ternary systems (three states, not two), the dynamics are far richer than binary variants: the neutral state (0) acts as a mediator that can absorb energy and create stable fixed points impossible in {-1, +1} automata.

Applications include:
- **Decentralized consensus**: agents avoid herd behavior
- **Pattern formation**: spontaneous symmetry breaking
- **Diversity maintenance**: preventing monoculture in agent populations
- **Complex systems theory**: studying the transition between order and chaos

## How It Works

### Update Rule

For each cell $i$ with neighbors $i-1$ and $i+1$ (periodic boundary), count occurrences:

$$c_v = |\{j \in \{i-1, i, i+1\} : x_j = v\}|, \quad v \in \{-1, 0, +1\}$$

The cell updates to the value with minimum count:

$$x_i(t+1) = \arg\min_{v} c_v$$

Ties are broken by selecting the smallest value ($-1 \prec 0 \prec +1$).

### Energy

The Ising-like energy of a configuration:

$$E = \frac{1}{N} \sum_{i=0}^{N-1} |x_i - x_{(i+1) \bmod N}|$$

- $E = 0$: uniform state (all same value)
- $E = 2$: maximally alternating ($+1, -1, +1, -1, \ldots$)

**Time complexity per step:** O(N) — each cell examined once.  
**Space complexity:** O(N) — one copy of the grid for double-buffering.

### Oscillators

A cell is an oscillator if $x_i(t) \neq x_i(t+1)$. The oscillating fraction measures dynamic instability:

$$f_{\text{osc}} = \frac{1}{N} \sum_i \mathbf{1}[x_i(t) \neq x_i(t+1)]$$

### Domain Walls

A domain wall exists between positions $i$ and $i+1$ when $|x_i - x_{i+1}| = 2$ (i.e., $+1$ adjacent to $-1$). Domain walls are the topological defects of the ternary ring.

### Cluster Count

Contiguous regions of identical value. For a ring of $N$ cells:

$$C = \sum_{i=0}^{N-1} \mathbf{1}[x_i \neq x_{(i-1) \bmod N}]$$

## Quick Start

```rust
use ternary_minority::*;

// Create a ring and evolve it
let mut grid = vec![1i8, -1, 0, 1, -1, 0, 1, -1, 0, 1];
let result = run_minority(&mut grid, 10, 100);
println!("Energy: {:.3}", result.energy);
println!("Zero fraction: {:.2}", result.final_zero_frac);
println!("Oscillating: {:.2}%", result.oscillating_frac * 100.0);

// Find oscillating cells
let mut grid2 = vec![0i8, 1, -1, 0, 1, -1];
let osc = find_oscillators(&mut grid2, 6);
println!("Oscillators at positions: {:?}", osc);

// Check stability
println!("Stable: {}", is_stable(&grid, 10));
```

## API

| Function | Description |
|---|---|
| `minority_step(&mut [i8], usize)` | One CA update step on the ring |
| `run_minority(&mut [i8], usize, usize) → MinorityResult` | Run N ticks, return final state metrics |
| `find_oscillators(&[i8], usize) → Vec<usize>` | Indices of cells that flip on next step |
| `domain_walls(&[i8], usize) → usize` | Count boundaries between +1 and -1 regions |
| `is_stable(&[i8], usize) → bool` | True if the grid doesn't change in one step |
| `MinorityResult` | Struct with `final_zero_frac`, `oscillating_frac`, `cluster_count`, `energy` |

## Architecture Notes

The minority CA connects to the ternary conservation law **γ + η = C** through its energy dynamics. Each ternary value $v \in \{-1, 0, +1\}$ contributes to either the constructive mass γ (for $v = +1$) or the inhibitory mass η (for $v = -1$), while $v = 0$ contributes to neither. The minority rule tends to *maximize* the number of distinct local values, driving the system toward equal representation of all three states. This creates a dynamic equilibrium where $\gamma \approx \eta \approx N/3$, and the neutral population acts as a buffer that absorbs perturbations without violating the conservation bound $C$.

Domain walls are the physical manifestation of energy gradients. Their count directly measures $E/2$ in the high-energy regime.

## References

- Wolfram, S. (2002). *A New Kind of Science.* Wolfram Media. (Elementary CA theory)
- Gács, P. (2001). *Deterministic Computations in Cellular Automata.* ECC.
- Nowak, M. A. (2006). *Evolutionary Dynamics.* Harvard University Press. (Minority games)
- Arthur, W. B. (1994). *Inductive Reasoning and Bounded Rationality: The El Farol Problem.* AEA Papers.

## License

MIT
