import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from math import comb

# Define ranges
n_range = range(1, 10_000)
k_range = range(1, 10_000)

# Initialize surface
prob_surface = np.zeros((len(n_range), len(k_range)))

# Compute probabilities
for i, n in enumerate(n_range):
    for j, k in enumerate(k_range):
        total_committee_size = 3 * k + 1
        if total_committee_size > 3 * n + 1:
            prob_surface[i, j] = np.nan
            continue

        c = comb(3 * n + 1, total_committee_size)
        c_star = 0
        for h in range(2 * k + 1, min(total_committee_size, 2 * n + 1) + 1):
            c_star += comb(2 * n + 1, h) * comb(n, total_committee_size - h)

        prob_surface[i, j] = c_star / c if c > 0 else np.nan

# Meshgrid for plotting
k_vals, n_vals = np.meshgrid(k_range, n_range)

# Plot
fig = plt.figure(figsize=(12, 8))
ax = fig.add_subplot(111, projection='3d')
surf = ax.plot_surface(k_vals, n_vals, prob_surface, cmap='plasma', edgecolor='k', linewidth=0.3)

ax.set_title(r'$\Pr[\text{Honest Committee}] = \frac{c^*}{c}$')
ax.set_xlabel('k')
ax.set_ylabel('n')
ax.set_zlabel('Probability')
fig.colorbar(surf, shrink=0.5, aspect=5)
plt.tight_layout()
plt.show()
