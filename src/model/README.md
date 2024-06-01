# Model module

The model contains the structs and validation logic of the internal representation.

## Structure

* `table_expr`: Contains the structs to define a table with its columns
* `column_rule`: Contains structs that define the different column level rules
* `rule_filter`: module for filtering tables before applying the rules
* `data_class`: contains structs for handling different data types and parsing them
* `rule_ext_config`: external rule config (empty struct at the moment) 