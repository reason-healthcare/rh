import re

with open("crates/rh-cql/src/emit/mod.rs", "r") as f:
    text = f.read()

# I want to add `pub mod operators;` if it's not present already. (It probably is from my earlier read, but just in case)
