import csv
import random

names = ["Joe", "Mary", "Emma", "Amir", "Cliff", "Sara", "John", "Lisa"]
domains = ["gmail.com", "yahoo.com", "outlook.com"]
statuses = ["active", "inactive"]

with open("test_large.csv", "w", newline="") as f:
    writer = csv.writer(f)
    writer.writerow(["name", "email", "age", "status"])
    for i in range(1_000_000):
        name = random.choice(names)
        email = f"{name.lower()}{i}@{random.choice(domains)}"
        age = random.randint(18, 65)
        status = random.choice(statuses)
        writer.writerow([name, email, age, status])

print("Done")