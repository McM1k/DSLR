import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns

file = "resources/dataset_train.csv"

data = pd.read_csv(file)
lines = ["Arithmancy", "Astronomy", "Herbology", "Defense Against the Dark Arts", "Divination", "Muggle Studies", "Ancient Runes", "History of Magic", "Transfiguration", "Potions", "Care of Magical Creatures", "Charms", "Flying"]
pal = dict(Gryffindor="red", Hufflepuff="yellow", Slytherin="green", Ravenclaw="blue")

g = sns.PairGrid(data, vars=lines, hue="Hogwarts House", palette=pal ,height=2, dropna=True)
g.map_lower(plt.scatter, alpha=0.3)
g.map_diag(plt.hist, alpha=0.6)
g.add_legend()
plt.savefig("pair.svg", format="svg")