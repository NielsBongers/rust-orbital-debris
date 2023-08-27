import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.cm as cm
from pathlib import Path

particle_file_paths = Path("results/data").glob("**/*")

for particle_file in particle_file_paths:
    df = pd.read_csv(particle_file)

    df = df[(df["t"] > 0) & (df["t"] % 500 == 0)]

    for index, row in df.iterrows():
        print(row["t"])
        plt.plot(row["x"], row["y"], "o", c=cm.jet(row["t"] / df["t"].max()))


earth = plt.Circle((0, 0), 6371 * 1000, color="forestgreen")
plt.gcf().gca().add_artist(earth)

plt.xlim([-10e6, 10e6])
plt.ylim([-10e6, 10e6])
plt.gca().set_aspect("equal")
plt.show()
