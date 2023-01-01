-- Your SQL goes here
CREATE TABLE "links" (
	"shortUrl"	TEXT NOT NULL UNIQUE,
	"redirectUrl"	TEXT,
	"photoUrl"	TEXT,
	"title"	TEXT,
	"description"	TEXT,
	PRIMARY KEY("shortUrl")
);