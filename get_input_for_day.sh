day="$1"
curl --cookie "session=$AOC_SESSION_TOKEN" "https://adventofcode.com/2022/day/$day/input" > "inputs/day$day.txt"
