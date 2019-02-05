import csv

with open('in.csv', newline='') as csvfile:
    reader = csv.DictReader(csvfile)
    for row in reader:
        year, make, model, description = row['year'], row['make'], row['model'], row['description']
        print(f"In {year}, {make} built the {model} model. It is a {description}.")
