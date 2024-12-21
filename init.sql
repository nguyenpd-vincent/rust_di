CREATE TABLE users (
                       id INT AUTO_INCREMENT PRIMARY KEY,
                       name VARCHAR(255) NOT NULL,
                       email VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE posts (
                       id INT AUTO_INCREMENT PRIMARY KEY,
                       user_id INT NOT NULL,
                       title VARCHAR(255) NOT NULL,
                       body TEXT NOT NULL,
                       FOREIGN KEY (user_id) REFERENCES users(id)
);

INSERT INTO users (name, email) VALUES
                                    ('nguyen', 'nguyen@example.com'),
                                    ('vincent', 'vincent@example.com');

INSERT INTO posts (user_id, title, body) VALUES
                                             (1, 'First Post', 'This is the body of the first post.'),
                                             (1, 'Second Post', 'This is the body of the second post.'),
                                             (2, 'Another Post', 'This is the body of another post.');