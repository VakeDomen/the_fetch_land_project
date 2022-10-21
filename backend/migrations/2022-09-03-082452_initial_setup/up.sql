-- Your SQL goes here-- Your SQL goes here
create table subscriptions
(
    id              varchar not null primary key,
    sale_type       varchar not null,
    user_id         varchar not null,
    sale_object_id  varchar not null,
    created         varchar not null
);

create table users
(
    id          varchar not null primary key,
    google_id   varchar not null,
    img         varchar not null,
    email       varchar not null,
    name        varchar null,
    phone       varchar null
);

create table sales
(
    id              varchar not null primary key,
    sale_type       varchar not null,
    user_id         varchar not null,
    sale_object_id  varchar not null,
    created         varchar not null,
    description     varchar not null,
    price           int not null,
    amount          int not null,
    contact_type    varchar not null,
    location        varchar not null,
    web_address     varchar not null
);
