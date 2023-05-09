#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Error: Please provide at least one glob pattern as an argument."
  exit 1
fi

IGNORED_DIRS=("./examples/reentrancy_guard/**" "./examples/test_helpers/**")

contains_element() {
  local element
  for element in "${@:2}"; do
    [[ "$element" == "$1" ]] && return 0
  done
  return 1
}

process_directory() {
  local dir=$1

  if contains_element "$(basename "$dir")" "${IGNORED_DIRS[@]}"; then
    return
  fi

  if [ -f "${dir}Cargo.toml" ]; then
    cd "$dir" || exit

    echo "Building contract in $dir"
    cargo contract build || exit

    echo "Running e2e-tests in $dir"
    cargo test --features e2e-tests || exit

    cd - || exit
  fi
}

for pattern in "$@"; do
  for dir in $pattern/*/; do
    process_directory "$dir"
  done
done
