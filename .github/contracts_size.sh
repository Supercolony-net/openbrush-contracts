#!/bin/bash

# create table header
REPORT=$'
| Contract Name | Merged | PR | Difference |
| --- | --- | --- | --- |
'

main_file_names=()
main_file_sizes=()
# get contract names and sizes from merged code
count=0
for item in $MAIN_DATA
do
  count=$((count+1))
  # even items are contract names, odd items are contracts size
  if [ $((count%2)) -eq 0 ]; then
    main_file_names+=("$item")
  else
    main_file_sizes+=("$item")
  fi
done

# get contract names and sizes from PR branch
pr_data=()
for item in $PR_DATA
do
  pr_data+=( "$item" )
done

# initialize pr_file_sizes by zeros
pr_file_sizes=()
for (( i=0; i<${#main_file_names[@]}; i++ ))
do
  pr_file_sizes+=("0")
done

# find contracts in PR branch, which exist in main branch
for ((i=0; i < ${#main_file_names[@]}; i+=1));
do
  # even items in pr_data are contract names, odd items are contracts size
  for ((j=0; j < ${#pr_data[@]}; j+=2));
  do
    if [[ "${main_file_names[i]}" == "${pr_data[j+1]}" ]]; then
      pr_file_sizes[i]=${pr_data[j]}
      break
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
for (( i=0; i<${#main_file_names[@]}; i++ ))
do
  REPORT="$REPORT | ${main_file_names[i]} | ${main_file_sizes[i]} | ${pr_file_sizes[i]} | ${dif_sizes[i]} |
  "
done

# write header to github environment
{
  echo "REPORT<<EOF"
  echo "$REPORT"
  echo "EOF"
} >> "$GITHUB_ENV"
