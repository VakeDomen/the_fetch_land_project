-- Your SQL goes here-- Your SQL goes here
create table users
(
    id          varchar not null primary key,
    username    varchar not null,
    password    varchar not null
);

create table sales
(
    id              varchar not null primary key,
    sale_type       varchar not null,
    user_id         varchar not null,
    sale_object_id  varchar not null,
    location_coords varchar not null,
    created         varchar not null,
    price           int not null,
    description     varchar not null
);
