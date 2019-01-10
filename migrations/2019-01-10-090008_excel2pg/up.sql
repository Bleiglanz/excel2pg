CREATE TABLE excel2pg (
id    SERIAL PRIMARY KEY,
file  TEXT   NOT NULL,
sheet TEXT   NOT NULL,
fdate TIMESTAMP NOT NULL, -- last modified date of file
idate TIMESTAMP NOT NULL, -- time of creation (identical for all rows)
s001  TEXT   NOT NULL,
s002  TEXT   NOT NULL,
s003  TEXT   NOT NULL
);
