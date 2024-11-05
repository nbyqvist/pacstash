create table upstream_mirrors (
    id integer primary key,
    upstream_id integer not null,
    url text not null,
    created_at integer not null default (strftime('%s', 'now')),
    updated_at integer,
    unique (upstream_id, url),
    foreign key (upstream_id) references upstreams(id)
) strict;
