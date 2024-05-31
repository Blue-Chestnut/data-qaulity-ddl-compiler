# Parser Module

The parser module contains the grammar definitions for parsing DDLx code. The parser uses the package 
[lalrpop](https://github.com/lalrpop/lalrpop) for lexing and parsing the code. The grammar is defined in the 
lalrpop files.

The main grammar is defined in `create_table.lalrpop`. There are two parts that are separated into different 
grammar files. This is due to lalrpop's internal lexing of keywords. There is no way to give precedence to the lexer
of lalrpop. This means that we have to manually exclude keywords from variable names. This is quite difficult as 
regex of lalrpop does not support backtracking and because SQL like keywords can be arbitrarily capitalized.

To circumvent this issue some keywords have been replaced with or prepended special characters, like the rule 
keywords (`-REGEX` needs the `-` to differ from variable name). For type declarations (like `FLOAT`, ...) a new
grammar was created in `data_class_parsing.lalrpop`. This new grammar takes care of parsing the datatype and also 
makes sure that the datatype is declared correctly, i.e. `INT` is invalid, but `INT(3)` is valid.

For rule filters a similar issue can easily arise and an additional grammar was introduced. This grammar can 
be found in `rule_filter_expr.lalrpop`.