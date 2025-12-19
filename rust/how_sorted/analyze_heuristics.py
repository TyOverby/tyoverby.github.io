#!/usr/bin/env python3
"""
Analyze heuristic agreement and generate a triangular heatmap.

This script reads the JSON output from the Rust program and creates
a heatmap showing how often each pair of heuristics agrees on which
sequence is "more sorted".
"""

import json

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.colors import LinearSegmentedColormap


def load_results(filename):
    """Load comparison results from JSON file."""
    with open(filename, "r") as f:
        return json.load(f)


def calculate_agreement_matrix(results):
    """
    Calculate pairwise agreement between heuristics.

    Returns a matrix where entry [i,j] is the percentage of times
    heuristic i and heuristic j agree on the outcome.
    """
    heuristics = [
        "exact_placement",
        "distance",
        "correct_neighbors",
        "runs",
        "lis",
    ]
    n = len(heuristics)
    agreement_matrix = np.zeros((n, n))

    total_comparisons = len(results)

    for i in range(n):
        for j in range(n):
            if i == j:
                agreement_matrix[i, j] = 100.0  # Always agrees with itself
            else:
                agreements = sum(
                    1 for r in results if r[heuristics[i]] == r[heuristics[j]]
                )
                agreement_matrix[i, j] = (agreements / total_comparisons) * 100

    return agreement_matrix, heuristics


def create_triangular_heatmap(
    agreement_matrix, heuristics, output_file="heuristic_agreement.png"
):
    """
    Create a triangular heatmap showing agreement between heuristics.

    Only the lower triangle is shown to avoid redundancy (since agreement is symmetric).
    The first row and last column are excluded since they contain no data in a
    lower-triangular layout.
    """
    n = len(heuristics)

    # Exclude first row and last column (they're always empty in lower triangle)
    # Row labels: heuristics[1:] (skip first)
    # Column labels: heuristics[:-1] (skip last)
    trimmed_matrix = agreement_matrix[1:, :-1]
    trimmed_n = n - 1

    # Create a masked array for the strictly upper triangle (k=1 excludes diagonal)
    mask = np.triu(np.ones_like(trimmed_matrix, dtype=bool), k=1)
    masked_data = np.ma.array(trimmed_matrix, mask=mask)

    # Create figure and axis
    fig, ax = plt.subplots(figsize=(10, 8))

    # Create custom colormap (white -> light blue -> dark blue)
    colors = ["#ffffff", "#e3f2fd", "#90caf9", "#42a5f5", "#1976d2", "#0d47a1"]
    n_bins = 100
    cmap = LinearSegmentedColormap.from_list("agreement", colors, N=n_bins)

    # Plot heatmap
    im = ax.imshow(masked_data, cmap=cmap, aspect="auto", vmin=0, vmax=100)

    # Add colorbar
    cbar = plt.colorbar(im, ax=ax, fraction=0.046, pad=0.04)
    cbar.set_label("Agreement (%)", rotation=270, labelpad=20, fontsize=12)

    # Set ticks and labels
    ax.set_xticks(np.arange(trimmed_n))
    ax.set_yticks(np.arange(trimmed_n))

    # Shorten labels for display
    short_labels = [
        "Exact\nPlacement",
        "Distance",
        "Correct\nNeighbors",
        "Runs",
        "LIS",
    ]

    # X-axis: all but last label, Y-axis: all but first label
    ax.set_xticklabels(short_labels[:-1], fontsize=10)
    ax.set_yticklabels(short_labels[1:], fontsize=10)

    # Rotate x-axis labels
    plt.setp(ax.get_xticklabels(), rotation=45, ha="right", rotation_mode="anchor")

    # Add text annotations for each cell
    for i in range(trimmed_n):
        for j in range(trimmed_n):
            if i >= j:  # Lower triangle including diagonal
                ax.text(
                    j,
                    i,
                    f"{trimmed_matrix[i, j]:.1f}%",
                    ha="center",
                    va="center",
                    color="black",
                    fontsize=9,
                )

    ax.set_title(
        "Heuristic Agreement Matrix\n(% of comparisons where heuristics agree)",
        fontsize=14,
        pad=20,
    )

    # Add grid
    ax.set_xticks(np.arange(trimmed_n) - 0.5, minor=True)
    ax.set_yticks(np.arange(trimmed_n) - 0.5, minor=True)
    ax.grid(which="minor", color="gray", linestyle="-", linewidth=0.5)
    ax.tick_params(which="minor", size=0)

    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches="tight")
    print(f"Heatmap saved to {output_file}")

    return fig, ax


def print_statistics(agreement_matrix, heuristics):
    """Print detailed statistics about heuristic agreements."""
    print("\n" + "=" * 60)
    print("HEURISTIC AGREEMENT STATISTICS")
    print("=" * 60)

    n = len(heuristics)

    # Print pairwise agreements
    print("\nPairwise Agreement Percentages:")
    print("-" * 60)
    for i in range(n):
        for j in range(i + 1, n):
            agreement = agreement_matrix[i, j]
            print(f"{heuristics[i]:20s} vs {heuristics[j]:20s}: {agreement:6.2f}%")

    # Find most and least similar pairs
    print("\nMost Similar Heuristics:")
    max_agreement = 0
    max_pair = None
    for i in range(n):
        for j in range(i + 1, n):
            if agreement_matrix[i, j] > max_agreement:
                max_agreement = agreement_matrix[i, j]
                max_pair = (i, j)

    if max_pair:
        i, j = max_pair
        print(f"  {heuristics[i]} ↔ {heuristics[j]}: {max_agreement:.2f}%")

    print("\nLeast Similar Heuristics:")
    min_agreement = 100
    min_pair = None
    for i in range(n):
        for j in range(i + 1, n):
            if agreement_matrix[i, j] < min_agreement:
                min_agreement = agreement_matrix[i, j]
                min_pair = (i, j)

    if min_pair:
        i, j = min_pair
        print(f"  {heuristics[i]} ↔ {heuristics[j]}: {min_agreement:.2f}%")

    print("\n" + "=" * 60)


def main():
    print("Loading results from heuristic_comparison_results.json...")
    results = load_results("heuristic_comparison_results.json")
    print(f"Loaded {len(results)} comparisons")

    print("\nCalculating agreement matrix...")
    agreement_matrix, heuristics = calculate_agreement_matrix(results)

    print_statistics(agreement_matrix, heuristics)

    print("\nGenerating triangular heatmap...")
    create_triangular_heatmap(agreement_matrix, heuristics)

    print("\nAnalysis complete!")


if __name__ == "__main__":
    main()
