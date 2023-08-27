import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.cm as cm
from pathlib import Path

particle_file_paths = Path("results/data").glob("**/*")

selected_names = ["particle 46"]

for particle_file in particle_file_paths:
    print(particle_file.stem in "".join(selected_names))

    for particle_name in selected_names:
        if particle_name == particle_file.stem:
            df = pd.read_csv(particle_file)
            plt.plot(df["x"], df["y"], "-")

earth = plt.Circle((0, 0), 6371 * 1000, color="forestgreen")
plt.gcf().gca().add_artist(earth)

plt.xlim([-10e6, 10e6])
plt.ylim([-10e6, 10e6])
plt.gca().set_aspect("equal")
# plt.axis("off")
plt.title("Orbital debris")
# plt.legend(
#     selected_names,
#     title="Deorbited particles (s)",
#     loc="upper right",
#     bbox_to_anchor=(1.45, 1),
# )
plt.tight_layout()
plt.savefig(
    "results/figures/27082023 - Rust orbital debris - almost a full orbit.png",
    dpi=300,
)
plt.show()
