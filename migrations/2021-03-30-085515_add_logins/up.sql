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
   if exists (select 1 from users.u where u.email = email)
   then
    return true;
   end if;
   return false;
end
$$
language plpgsql;
