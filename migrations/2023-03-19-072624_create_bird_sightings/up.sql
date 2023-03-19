-- Your SQL goes here
CREATE TABLE bird_sighting (
	id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
	bird_id INT NOT NULL,
	sighting_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	sighting_location VARCHAR(255),
	additional_information TEXT
);