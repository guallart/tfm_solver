import argparse
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
import matplotlib.colors as mcolors


def plot_heatmap(file_path):
    matrix = np.loadtxt(file_path)
    print(matrix[:, -1])

    plt.figure(figsize=(8, 6))
    colors = ["blue", "white", "red"]
    cmap = mcolors.LinearSegmentedColormap.from_list("custom_colormap", colors, N=256)
    sns.heatmap(matrix, cmap=cmap, annot=False, center=0)
    plt.show()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Plot a matrix from a space-separated file as a heatmap."
    )
    parser.add_argument("file", help="Path to the file containing the matrix")
    args = parser.parse_args()

    plot_heatmap(args.file)
