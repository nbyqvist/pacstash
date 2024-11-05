create table upstreams (
    id integer primary key,
    name text not null unique,
    upstream_type text not null,
    created_at integer not null default (strftime('%s', 'now')),
    updated_at integer
) strict;
