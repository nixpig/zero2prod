create table if not exists subscriptions(
	id serial primary key,
	email text not null unique,
	name text not null,
	subscribed_at timestamp not null default now()
);
