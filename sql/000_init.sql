CREATE TABLE [IF NOT EXISTS] photos.metadata (
   id BLOB PRIMARY KEY,
   original_name TEXT NOT NULL,
   mime_type TEXT NOT NULL,
   
);