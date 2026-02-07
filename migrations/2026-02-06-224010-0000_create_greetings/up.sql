CREATE TABLE greetings (
  id SERIAL PRIMARY KEY,
  greeting VARCHAR NOT NULL,
  language VARCHAR NOT NULL
);

CREATE TABLE access_tokens (
    id SERIAL PRIMARY KEY,
    token VARCHAR NOT NULL
);
    