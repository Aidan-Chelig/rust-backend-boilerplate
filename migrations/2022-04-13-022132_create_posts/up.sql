CREATE TABLE posts (
    id int PRIMARY KEY AUTO_INCREMENT,
    title varchar(255) NOT NULL,
    text varchar(255) NOT NULL,
    published boolean NOT NULL DEFAULT 0
);
