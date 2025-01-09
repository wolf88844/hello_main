#pgSql
create table users(
    id serial primary key,
    username varchar(255) not null,
    password varchar(255) not null,
    status int not null default 1,
    created timestamp with time zone default current_timestamp,
    updated timestamp with time zone default current_timestamp,
    last_login timestamp
)

create unique index idx_users_username on users(username);

create table posts(
    id serial primary key,
    author_id int not null,
    slug varchar(255) not null,
    title varchar(255) not null,
    content text not null,
    status int not null default 1,
    created timestamp with time zone default current_timestamp,
    updated timestamp with time zone default current_timestamp,
	foreign key (author_id) references users(id)
)

create unique index idx_posts_slug on posts(slug);

create or replace function updated_at_column()
returns trigger as $$
begin
    new.updated = current_timestamp;
    return new;
end
$$ language plpgsql;

create trigger updated_at_column
before update on users
for each row execute procedure updated_at_column();

create trigger updated_at_column
before update on posts
for each row execute procedure updated_at_column();

