-- Your SQL goes here
create table cameras (
  id serial primary key not null,
  name text not null unique
);
