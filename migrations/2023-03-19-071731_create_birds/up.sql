-- Your SQL goes here
CREATE TABLE bird (
	id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
	name VARCHAR(255) NOT NULL,
	scientific_name VARCHAR(255) NOT NULL,
	commonwealth_status VARCHAR(255) NOT NULL
);

INSERT INTO bird (name, scientific_name, commonwealth_status) VALUES
("Black-eared Miner", "Manorina melanotis", "Critically Endangered"),
("Eastern Bristlebird", "Dasyornis brachypterus", "Endangered"),
("Swift Parrot", "Lathamus discolor", "Endangered"),
("Australasian Bittern", "Botaurus poiciloptilus", "Endangered"),
("Southern Giant Petrel", "Macronectes giganteus", "Endangered"),
("Star Finch (eastern), Star Finch (southern)", "Neochmia ruficauda ruficauda", "Endangered"),
("Gould's Petrel", "Pterodroma leucoptera leucoptera", "Endangered"),
("Mallee Emu-wren", "Stipiturus mallee", "Endangered"),
("Coxen's Fig-Parrot", "Cyclopsitta diophthalma coxeni", "Critically Endangered"),
("Black-throated Finch (southern)", "Poephila cincta cincta", "Endangered"),
("Chatham Albatross", "Thalassarche eremita", "Endangered"),
("Grey Grasswren (Bulloo)", "Amytornis barbatus barbatus", "Endangered"),
("Australian Painted Snipe", "Rostratula australis", "Endangered"),
("Amsterdam Albatross", "Diomedea exulans amsterdamensis", "Endangered"),
("Northern Royal Albatross", "Diomedea epomophora sanfordi", "Endangered"),
("Tristan Albatross", "Diomedea exulans exulans", "Endangered");