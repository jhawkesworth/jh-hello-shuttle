CREATE TABLE IF NOT EXISTS FILES(
   id serial PRIMARY KEY,
   uri text NOT NULL ,
   content_type text NOT NULL ,
   file bytea NOT NULL
);