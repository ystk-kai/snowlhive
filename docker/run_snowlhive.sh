#!/usr/bin/env bash

set -eo pipefail

git_repository_dir="/tmp/$(basename $GIT_REPOSITORY_URL .git)"
input_dir=$git_repository_dir

if [[ -n $INPUT_PATH ]]; then
    input_dir="$input_dir/$INPUT_PATH"
fi

if [[ ! -d ./rust-lang_book ]]; then
    git clone --depth 1 $GIT_REPOSITORY_URL $git_repository_dir
fi

snowlhive --verbose -n $OUTPUT_NAME $input_dir
