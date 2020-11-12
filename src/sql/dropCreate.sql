drop schema if exists finance cascade;
create schema if not exists finance;
create table finance.users
(
    id serial primary key,
    name varchar(50)
);
create table finance.category
(
    id serial primary key,
    name varchar(80)
);
create table finance.expense
(
    id serial primary key,
    expense_date timestamptz,
    price numeric,
    description varchar(255),
    category_id bigint references finance.category(id),
    user_id bigint references finance.users(id)
);
create  table finance.planned_expense
(
    id serial primary key,
    priority int, -- can be from 1 to  10
    name varchar(80),
    category_id bigint references finance.category(id),
    user_id bigint references finance.users(id)
);