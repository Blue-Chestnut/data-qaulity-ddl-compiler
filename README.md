# data-quality-ddl-compiler

In data engineering we have four big concepts of data: 1) Table definition, 2) transformation Logic, 3)
validation logic, and 4) lineage. At the moment, these four areas are entirely individual and only loosely coupled. 
There is no easy way to validate if the transformation output adheres to the defined schema before running the
transformation. There is no way of checking if the validation logic references existing tables and columns automatically.
Finally, there is no way to check if the logic actually adheres to the lineage. There are some tools that 
help with some of the coupling of the fields, but they are often specific to certain packages and require assets to
be materialized. 

The DataQualityDDLCompiler couples these fields together largely at compile time. This means that if the code does not
compile, there is some error in the definition of the logic, schema, or validation.

## Functionality

### DDLx

DDLx is an extension to SQLs data domain language (DDL) to generate the table schema in conjunction with the validation
rules. The language supports most concepts that DDL supports as well.

You can generate a table just like in DDL:
```SQL
CREATE TABLE IF NOT EXISTS Test {
  Id VARCHAR(10),
  Price FLOAT(3,8) PRIMARY KEY
};
```
This is a valid DDL definition and could be used to define a simple table `Test`. With DDLx you can add
various additional things:
```SQL
CREATE TABLE IF NOT EXISTS Test "This table is an example" (1){
    Name Varchar(10) { (2)
        -LIKE "%test%",
        -REGEX "[0-9]*test[0-9]*",
        -CONTAINS "test" 0.9,
        -NOT_EMPTY,
        -UNIQUE | Price > 10 (3)},
    Price FLOAT(3,8) PRIMARY KEY (4)
};
```
The `(<number>)` are to reference certain parts in the code and are not valid DDLx syntax.
You can add a freetext alias to you tables (1).
You can define validation rules that are applied to the corresponding column (2).
You can filter the table to apply rules to a subset of the table (3).

DDL keywords like `PRIMARY KEY`, `FOREIGN KEY`, or `NOT NULL` automatically generate checks that correspond to 
the keywords implied rules. Additionally, the compiler generates a type check for each column.

### Type Safety in DDLx

DDLx is type safe and checks this at compile time. This means that if 


## Repo Structure

* examples: examples of the usage
* src: code
  * compiler: code for compiling from internal representation to external
  * model: code defining and validating the internal representation
  * parser: code related to lexing and parsing the code to the internal language
    * lalrpop files: files define the grammar of the programming language
    * tests.rs files are tests for the lalrpop files
* templates: templates for generating the output code
* test_data: data for running tests, can be templates, ...
* valid_column_rule_derive: rule derive definition for valid_column_rule trait

## Usage

### CLI 

`cargo run -- -t py-deequ -f "examples/dq-ddl-examples/create-table-with-check.sqlx" -o test.py`

## Ways of Working

### Pushing Code

* make sure to run `cargo fmt` and `cargo clippy` before raising a PR

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



