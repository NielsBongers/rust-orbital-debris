import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv("results/data/ISS.csv")
plt.plot(df["x"], df["y"], "-")

df = pd.read_csv("results/data/Satellite.csv")
plt.plot(df["x"], df["y"], "-")

df = pd.read_csv("results/data/Earth.csv")
plt.plot(df["x"], df["y"], "x-")

plt.show()
