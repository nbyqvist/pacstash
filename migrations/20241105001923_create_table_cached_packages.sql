create table cached_packages (
    id integer primary key,
    upstream_id integer not null,
    repo text not null,
    arch text not null,
    filename text not null,
    upstream_mirror_id integer not null,
    created_at integer not null default (strftime('%s', 'now')),
    updated_at integer,
    unique (upstream_id, repo, arch, filename),
    foreign key (upstream_id) references upstreams(id),
    foreign key (upstream_mirror_id) references upstream_mirrors(id)
) strict;
