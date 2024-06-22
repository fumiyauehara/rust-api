-- up
CREATE TABLE test_migrations (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL
);

-- down
DROP TABLE test_migrations;

