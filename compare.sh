# #!/bin/bash

# Record the start time for graph-visualize
start_time_graph=$(date +%s.%N)

for i in {1..1000}; do
    graph-visualize/target/debug/graph-visualize > /dev/null
    echo "Iteration $i" > /dev/null
done

# Record the end time for graph-visualize
end_time_graph=$(date +%s.%N)

# Calculate the total elapsed time for graph-visualize
duration_time=$(echo "$end_time_graph - $start_time_graph" | bc)


# why need to pip first?
# pip install asciichartpy

# Record the start time for python-visualize
# start_time_python=$(date +%s.%N)

# for i in {1..1000000}; 
# do
#     python3 python-visualize/main.py  > /dev/null
# done

# # Record the end time for python-visualize
# end_time_python=$(date +%s.%N)

# # Calculate the total elapsed time for python-visualize
# elapsed_time_python=$(echo "$end_time_python - $start_time_python" | bc)

echo "Total time duration for graph-visualize: $duration_time seconds"
# echo "Total time duration for python-visualize: $elapsed_time_python seconds"