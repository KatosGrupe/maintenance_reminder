-- Your SQL goes here
create table users (
  id serial primary key,
  email text not null unique,
  password text not null
);

create type user_permission as enum ('technician', 'manager', 'analyst');

create table user_permissions (
  user_id Integer references users(id),
  permission user_permission,
  primary key (user_id, permission)
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

-- populate initial server values (TESTING PURPOSES ONLY)
call add_user('ignas@kata.lt', 'Test123');
insert into user_permissions
values (1, 'technician');
