import time
import timeit
import psutil
from .main import vis


def test():
    start = timeit.default_timer()
    vis()
    end = timeit.default_timer()
    duration_microseconds = (end - start) * 1_000_000  # Convert to microseconds

    cpu_usage = psutil.cpu_percent()
    mem_usage = psutil.virtual_memory()

    print("Total execution time: {:.2f} microseconds".format(duration_microseconds))
    print(f"Memory Usage: {mem_usage.percent}%")
    print(f"CPU Usage: {cpu_usage}%")


if __name__ == "__main__":
    print("Python script result and info are:")
    test()