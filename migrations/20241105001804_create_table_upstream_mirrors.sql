create table upstream_mirrors (
    id integer primary key not null,
    upstream_id integer not null,
    -- Url as present in *mirrorlist including $arch/$repo variables
    url text not null,
    created_at integer not null default (strftime('%s', 'now')),
    updated_at integer,
    unique (upstream_id, url),
    foreign key (upstream_id) references upstreams(id)
) strict;
