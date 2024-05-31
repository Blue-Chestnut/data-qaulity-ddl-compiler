

CREATE TABLE IF NOT EXISTS Inventory.Books {
    Id INT PRIMARY KEY,
    Title VARCHAR(100) NOT NULL,
    Author VARCHAR(100) NOT NULL,
    Price DECIMAL(10, 2) CONSTRAINT price_positive CHECK (Quantity >= 0),
    Quantity INT NOT NULL CONSTRAINT qty_positive CHECK (Quantity >= 0),
}


CREATE TABLE IF NOT EXISTS Inventory.Books {
    Id INT,
    Title VARCHAR,
}