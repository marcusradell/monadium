create table if not exists challenges (
      id bigint unsigned not null auto_increment,
      name text not null,
      content text not null,
      primary key (id)
);