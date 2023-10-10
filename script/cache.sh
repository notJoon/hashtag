

# CACHE_FILE="target/hashtag_cache.txt"
# CACHE_TIMESTAMP_FILE="target/hashtag_cache_timestamp.txt"
# MAX_AGE=3600

# current_timestamp=$(date +%s)

# if [ -f "$CACHE_TIMESTAMP_FILE"] && [ -f "$CACHE_FILE" ]; then
#     cached_data=$(cat "$CACHE_FILE")
#     cached_timestamp=$(cat "$CACHE_TIMESTAMP_FILE")
#     age=$((current_timestamp - cached_timestamp))

#     # if cached data has generated less than valid time, use it
#     if [ $age -lt $MAX_AGE ]; then
#         # TODO load cached data
#         echo "loaded cached data"
#         echo "$cached_data"
#         exit 0
#     fi
# fi

# # generate new cache data, if it doesn't exist or is too old
# echo "generating new cache data"
# if [ -z "$cached_data" ]; then
#     new_data=$(cargo test -- --list | grep -E "^\s*test\s+.*\s+\.\.\.\s+ok$")
# fi