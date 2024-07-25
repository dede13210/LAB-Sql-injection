CREATE DATABASE IF NOT EXISTS clientdb;

USE clientdb;

CREATE TABLE IF NOT EXISTS clients (
    id INT AUTO_INCREMENT PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL,
    phone VARCHAR(20),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


INSERT INTO clients (first_name, last_name, email, phone) VALUES
('John', 'Doe', 'jodoe@mail.fr', '1234567890'),
('Jane', 'Doe', 'jani@mail.com', '0987654321'),
('marie', 'Opkins', 'marie@gmail.fr', '1234567890'),
('jean', 'Dupont', 'jdfjfj@mail.fr', '0987654321'),
('paul', 'Dupuis', 'depuis@mail.fr', '1234567890'),
('pierre', 'Dupond', 'pierre@roule.com', '0987654321');