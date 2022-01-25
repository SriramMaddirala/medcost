CREATE TABLE reviews (
  id SERIAL PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  hospital_name TEXT NOT NULL,
  doctor_name TEXT NOT NULL,
  doctor_type TEXT NOT NULL,
  service_rendered TEXT NOT NULL,
  service_cost BIGINT NOT NULL
);
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL,
  passwd TEXT NOT NULL,
  email TEXT NOT NULL
);