# #!/bin/bash

# why need to pip first?
# pip install asciichartpy

# Record the start time for python-visualize
start_time_python=$(date +%s.%N)

for i in {1..1000}; 
do
    python3 python-visualize/main.py  > /dev/null
    echo "Iteration $i" > /dev/null
done

# Record the end time for python-visualize
end_time_python=$(date +%s.%N)

# Calculate the total elapsed time for python-visualize
duration_time_python=$(echo "$end_time_python - $start_time_python" | bc)

echo "Total time duration for python-visualize: $duration_time_python seconds"