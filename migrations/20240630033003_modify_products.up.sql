-- Add up migration script here
alter table products add column pharmacy_id int unsigned not null default 1;
