-- Add up migration script here
CREATE TABLE IF NOT EXISTS products (
    product_id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    product_name VARCHAR(50) NOT NULL,
    category VARCHAR(30) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    stock INT UNSIGNED NOT NULL,
    PRIMARY KEY (product_id)
) ENGINE=InnoDB;

INSERT INTO products (product_name, category, price, stock) VALUES
('Product A', 'Category 1', 10.00, 100),
('Product B', 'Category 2', 20.00, 50),
('Product C', 'Category 1', 15.00, 200),
('Product D', 'Category 3', 25.00, 30);
