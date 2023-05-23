create table if not exists users (
  user_id serial primary key,
  user_fingerprint VARCHAR(50) not null,
  user_public_key VARCHAR(2500) not null,
  is_admin boolean not null
);
