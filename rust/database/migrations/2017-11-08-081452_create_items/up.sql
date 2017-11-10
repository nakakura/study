CREATE TABLE items (
  id INT UNSIGNED AUTO_INCREMENT NOT NULL PRIMARY KEY,
  title varchar(255) NOT NULL UNIQUE,
  owner TEXT NOT NULL,
  borrower TEXT,
  registered_date DateTime,
  due_date DateTime
) COLLATE utf8mb4_bin;

