-- Your SQL goes here
create table users (
  id serial primary key,
  email text not null unique,
  password text not null
);

create procedure add_user(email text, password text)
language sql
as $$

   insert into users(email, password)
   values (email, crypt(password, gen_salt('bf')));

$$;

create function check_user(email text, password text)
returns boolean as $$
begin
   if exists (select 1 from users u where u.email = email and u.password = crypt(password))
   then
    return true;
   end if;
   return false;
end
$$
language plpgsql;


call add_user('ignas@kata.lt', 'Test123');

select * from users;


select 1 from users u
where u.email = 'ignas@kata.lt'
      and u.password = crypt('Test123', u.password);
