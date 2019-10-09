import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns

file = "resources/dataset_train.csv"

data = pd.read_csv(file)
pal = dict(Gryffindor="red", Hufflepuff="yellow", Slytherin="green", Ravenclaw="blue")
g = sns.PairGrid(data,
    vars=[
        "Arithmancy",
        "Astronomy",
        "Herbology",
        "Defense Against the Dark Arts",
        "Divination",
        "Muggle Studies",
        "Ancient Runes",
        "History of Magic",
        "Transfiguration",
        "Potions",
        "Care of Magical Creatures",
        "Charms",
        "Flying"
    ],
    hue="Hogwarts House",
    palette=pal,
    diag_kind="hist")
g.map_lower(sns.lmplot)
g.map_diag(sns.distplot)
g.map_upper(sns.kdeplot)
g.add_legend()
plt.savefig("pair.svg", format="svg")