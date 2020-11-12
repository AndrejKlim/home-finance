create schema finance;
create table finance.user
(
    id serial primary key,
    name varchar(50)
);
create table finance.category
(
    id serial primary key,
    name varchar(80)
);
create table finance.purchase
(
    id serial primary key,
    purchase_date timestamp,
    price numeric,
    description varchar(255),
    category_id bigint references finance.category(id),
    user_id bigint references finance.user(id)
);
create  table finance.planned_purchase
(
    id serial primary key,
    priority int, -- can be from 1 to  10
    name varchar(80),
    category_id bigint references finance.category(id),
    user_id bigint references finance.user(id)
);