import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.cm as cm
from pathlib import Path

particle_file_paths = Path("results/data").glob("**/*")


has_legend = False
legend_list = []

plotting_period = 500
starting_time = 0
end_time = 5000

for particle_file in particle_file_paths:
    df = pd.read_csv(particle_file)

    # plt.plot(df["x"], df["y"], "-x")
    # plt.show()

    df = df[
        (df["t"] >= starting_time)
        & (df["t"] <= end_time)
        & (df["t"] % plotting_period == 0)
    ]

    for index, row in df.iterrows():
        if not has_legend:
            legend_list.append(int(row["t"]))
        plt.plot(row["x"], row["y"], "o", c=cm.jet(row["t"] / df["t"].max()))

    if not has_legend:
        has_legend = True


earth = plt.Circle((0, 0), 6371 * 1000, color="forestgreen")
plt.gcf().gca().add_artist(earth)

plt.xlim([-10e6, 10e6])
plt.ylim([-10e6, 10e6])
plt.gca().set_aspect("equal")
# plt.axis("off")
plt.title("Orbital debris")
plt.legend(
    legend_list,
    title="Simulation time (s)",
    loc="upper right",
    bbox_to_anchor=(1.45, 1),
)
plt.tight_layout()
plt.savefig("results/figures/simulation_results.png", dpi=300)
plt.show()
