#!/bin/sh
set -e

for example in $(ls examples/quicksilver);
do
    name=$(echo $example | cut -f 1 -d '.');
    cargo web check --example $name;
done
