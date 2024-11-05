create table upstreams (
    id integer primary key,
    -- `name` is the url param that the upstream will be accessed with
    name text not null unique,
    upstream_type text not null,
    created_at integer not null default (strftime('%s', 'now')),
    updated_at integer
) strict;
