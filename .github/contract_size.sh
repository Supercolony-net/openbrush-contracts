#!/bin/bash

# create table header
REPORT=$'
| Contract Name | Main | PR | Diff |
| --- | --- | --- | --- |
'

file_names=()
main_file_sizes=()
# get contract names and sizes from main branch
count=0
for item in $MAIN_DATA
do
  count=$((count+1))
  if [ $((count%2)) -eq 0 ]; then
    file_names+=("$item")
  else
    main_file_sizes+=("$item")
  fi
done

# get contract names and sizes from PR branch
echo "$PR_DATA"
pr_data=()
for item in $PR_DATA
do
  pr_data+=( "$item" )
done

# find contracts in PR branch, which exist in main branch
pr_file_sizes=()
for ((i=0; i < ${#pr_data[@]}; i+=2));
do
  for name in "${file_names[@]}"
  do
    if [[ $name == "${pr_data[i+1]}" ]]; then
      pr_file_sizes+=("${pr_data[i]}")
    fi
  done
done

# calculate the size difference
dif_sizes=()
for (( i=0; i<${#main_file_sizes[@]}; i++ ))
do
  dif_sizes+=( $(( pr_file_sizes[i] - main_file_sizes[i] )) )
done

# finish report table
for (( i=0; i<${#file_names[@]}; i++ ))
do
  REPORT="$REPORT | ${file_names[i]} | ${main_file_sizes[i]} | ${pr_file_sizes[i]} | ${dif_sizes[i]} |
  "
done

# write header to github environment
{
  echo "REPORT<<EOF"
  echo "$REPORT"
  echo "EOF"
} >> "$GITHUB_ENV"
