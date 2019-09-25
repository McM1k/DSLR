# DSLR
42 project about data science and logistic regression

The goal is to recreate the sorting hat from Harry Potter.

What is asked to do :

- recreate the describe method for a feature of the dataset, along with standard deviation, percentiles, means, counts...
- plot the data in various ways (histograms, scatter points, pair plotting)
- train the program with a dataset so it can select the right house for the new students afterwards
- of course estimate the houses of new students

Usage : 
- `-p [testfile_path]` to predict houses for new students
- `-t [trainfile_path]` to train the sorting hat 2.0
  - `-v [describe|histogram|scatter|pair] (optional)` to visualise data
    - `describe` shows count, mean, std deviation and percentiles of each feature
    - `histogram` shows the feature where data is the most homogenous between houses
    - `scatter` shows the two similar features
    - `pair` shows a scatter plot matrix