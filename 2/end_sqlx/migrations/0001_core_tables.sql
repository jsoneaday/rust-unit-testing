create table message (
    "id" bigserial primary key,
    "created_at" timestamptz(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "title" varchar(50) NOT NULL,
    "body"  varchar(140)
);
