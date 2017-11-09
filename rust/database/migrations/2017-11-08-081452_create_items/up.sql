CREATE TABLE items (
  title VARCHAR(255) NOT NULL PRIMARY KEY,
  owner TEXT NOT NULL,
  borrower TEXT,
  due_date DateTime
);

