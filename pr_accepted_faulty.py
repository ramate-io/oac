import numpy as np
import matplotlib.pyplot as plt
from math import comb
from scipy.special import gammaln, logsumexp


def binom_safe(n, k, threshold=1000):
    """
    Computes binomial coefficient C(n, k).
    Uses math.comb for small n and gamma approximation for large n.
    """
    if k < 0 or k > n:
        return 0.0
    if n < threshold:
        return comb(n, k)
    return np.exp(gammaln(n + 1) - gammaln(k + 1) - gammaln(n - k + 1))


def log_binom(n, k):
    if k < 0 or k > n:
        return -np.inf
    return gammaln(n + 1) - gammaln(k + 1) - gammaln(n - k + 1)


# Fixed k/n ratios to examine
k_ratio_values = [0.3, 0.1, 0.01, 0.001]
n_values = np.logspace(np.log2(2), np.log2(32), num=400, dtype=int)  # Logarithmic steps

# Dictionary to store Pr[Accepted Faulty] for each fixed k/n
pr_honest_by_ratio = {ratio: [] for ratio in k_ratio_values}

# Compute Pr[Accepted Faulty] for each (k/n, n)
for ratio in k_ratio_values:
    for n in n_values:
        k = round(n * ratio)
        if k < 1 or k > n:
            pr_honest_by_ratio[ratio].append(np.nan)
            continue

        total_nodes = 3 * n + 1
        honest_nodes = 2 * n + 1
        faulty_nodes = n
        committee_size = 3 * k + 1

        if committee_size > total_nodes:
            pr_honest_by_ratio[ratio].append(np.nan)
            continue

        # Compute log of total combinations
        log_total_combinations = log_binom(total_nodes, committee_size)

        # Sum log terms of faulty-majority committees
        log_faulty_terms = []
        for h in range(2 * k + 1, min(committee_size, faulty_nodes) + 1):
            log_term = log_binom(faulty_nodes, h) + log_binom(honest_nodes, committee_size - h)
            log_faulty_terms.append(log_term)

        log_accepted_faulty = logsumexp(log_faulty_terms)

        # Compute final probability safely
        log_pr = log_accepted_faulty - log_total_combinations
        pr = np.exp(log_pr)
        pr_honest_by_ratio[ratio].append(pr)

# Plotting
plt.figure(figsize=(12, 7))

for ratio, probs in pr_honest_by_ratio.items():
    line, = plt.plot(n_values, probs, color='black')
    # Annotate directly on the curve
    idx = np.argmax(np.isfinite(probs)) + len(probs) // 3
    if idx < len(n_values):
        x = n_values[idx + 25]
        y = probs[idx + 40]
        plt.text(x, y, f'k/n = {ratio:.3f}', fontsize=12, color='black', ha='left', va='bottom', rotation=0,  bbox=dict(facecolor='white', edgecolor='none', boxstyle='round,pad=0.2'))

# Reference security lines
plt.axhline(2**-128, color='black', linestyle=':', linewidth=1)
plt.axhline(2**-256, color='black', linestyle='-.', linewidth=1)
plt.axhline(2**-512, color='black', linestyle='--', linewidth=1)
plt.text(n_values[10], 2**-128, "128-bit", fontsize=10, color='black', va='bottom', ha='left')
plt.text(n_values[10], 2**-256, "256-bit\n(PQ 128-bit)", fontsize=10, color='black', va='bottom', ha='left')
plt.text(n_values[10], 2**-512, "512-bit\n(PQ 256-bit)", fontsize=10, color='black', va='bottom', ha='left')

plt.yscale("log", base=2)
plt.xscale("log", base=2)
plt.title("Pr[Accepted Faulty] vs Number of Participants (log scale)", fontsize=14)
plt.xlabel("Number of Participants (log scale)")
plt.ylabel("Pr[Accepted Faulty] (log scale)")
plt.tight_layout()

plt.gca().spines['top'].set_visible(False)
plt.gca().spines['right'].set_visible(False)
plt.gca().grid(False)

plt.rcParams.update({
    "font.family": "monospace",  # or "Courier New", "Courier"
    "font.monospace": "Courier",  # explicitly prioritize Courier
})

plt.show()