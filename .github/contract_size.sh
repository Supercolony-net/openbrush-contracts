#!/bin/bash

# create table header
REPORT=$'
| Contract Name | Merged | Main | Difference |
| --- | --- | --- | --- |
'

main_file_names=()
main_file_sizes=()
# get contract names and sizes from main branch
count=0
for item in $MAIN_DATA
do
  count=$((count+1))
  if [ $((count%2)) -eq 0 ]; then
    main_file_names+=("$item")
  else
    main_file_sizes+=("$item")
  fi
done

# get contract names and sizes from source branch
source_data=()
for item in $SOURCE_DATA
do
  source_data+=( "$item" )
done

# initialize source_file_sizes by zeros
source_file_sizes=()
for (( i=0; i<${#main_file_names[@]}; i++ ))
do
  source_file_sizes+=("0")
done

# find contracts in source branch, which exist in main branch
for ((i=0; i < ${#main_file_names[@]}; i+=1));
do
  for ((j=0; j < ${#source_data[@]}; j+=2));
  do
    if [[ "${main_file_names[i]}" == "${source_data[j+1]}" ]]; then
      source_file_sizes[i]=${source_data[j]}
      break
    fi
  done
done

# calculate the size difference
dif_sizes=()
for (( i=0; i<${#main_file_sizes[@]}; i++ ))
do
  dif_sizes+=( $(( source_file_sizes[i] - main_file_sizes[i] )) )
done

# finish report table
for (( i=0; i<${#main_file_names[@]}; i++ ))
do
  REPORT="$REPORT | ${main_file_names[i]} | ${main_file_sizes[i]} | ${source_file_sizes[i]} | ${dif_sizes[i]} |
  "
done

# write header to github environment
{
  echo "REPORT<<EOF"
  echo "$REPORT"
  echo "EOF"
} >> "$GITHUB_ENV"
