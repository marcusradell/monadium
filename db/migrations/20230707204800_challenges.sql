create schema if not exists challenges;

create table if not exists challenges.challenges (
      id uuid not null,
      name text not null,
      content text not null,
      primary key (id)
);