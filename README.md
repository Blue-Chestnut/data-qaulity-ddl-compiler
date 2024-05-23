# data-qaulity-ddl-compiler
A compiler for extending DDLs to include data quality rules.

## Usage

### CLI 

`cargo run -- -t py-deequ -f "examples/dq-ddl-examples/create-table-with-check.sqlx" -o test.py`

## Language Definition

### Formal Grammar

### Supported Rules

#### Single Column Rules

* Regex
* Range
* Aritmetic
* At Least Once of Range/Set
* Switch Case

* Standard
    * Uniqueness
    * Non-null

#### Multiple Column Rules of One Table

* Aritmetic

#### Multiple Column Rules of Multiple Tables

* Aritmetic
* Aggregation Comparison
* Referential Integrity

### Examples

## Ways of Working

### Pushing Code

* make sure to run `cargo fmt` and `cargo clippy` before raising a PR

