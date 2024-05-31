Create table if not exists Test {
    Id Varchar(10) {
        -LIKE "%test%" | Price > 10,
        -REGEX "[0-9]*test[0-9]*",
        -CONTAINS "test" 0.9 | Price > 10,
        -not_empty,
        -unique},
    Price FLOAT(3,8) PRIMARY KEY
};