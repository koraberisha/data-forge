from ai_data_fabric import AIDataFabric

def benchmark():
    fabric = AIDataFabric()
    data = list(range(1, 1000001))  # 1 million data points
    fabric.add_data_source("benchmark_data", data, [len(data)])

    # Measure time for parallel transformation
    import time
    start_time = time.time()
    fabric.transform("benchmark_data")
    parallel_duration = time.time() - start_time
    print(f"Parallel transformation took {parallel_duration:.2f} seconds")

if __name__ == "__main__":
    benchmark()
