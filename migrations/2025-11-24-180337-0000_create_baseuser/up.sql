-- Your SQL goes here
CREATE TABLE "users"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"username" TEXT NOT NULL,
	"password" TEXT NOT NULL
);

CREATE TABLE "profiles"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"user_id" INTEGER NOT NULL,
	"hash" TEXT NOT NULL,
	"name" TEXT NOT NULL
);

