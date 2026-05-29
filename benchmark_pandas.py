import pandas as pd
import time

start = time.time()
df = pd.read_csv("test_large.csv")
df = df[df["status"] == "active"][["name", "email", "age"]]
df.to_csv("result_pandas.csv", index=False)
end = time.time()
print(f"Pandas time: {end - start:.2f}s")