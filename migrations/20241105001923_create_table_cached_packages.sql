create table cached_packages (
    id integer primary key not null,
    upstream_id integer not null,
    -- $repo var from url
    repo text not null,
    -- $arch/$arch_v3/$arch_v4 var from url
    arch text not null,
    filename text not null,
    upstream_mirror_id integer not null,
    download_count integer not null default (0),
    last_downloaded_at integer,
    -- For a cached package, the created_at column is the time the package was first cached
    -- This will be used later for rules around cache eviction
    created_at integer not null default (strftime('%s', 'now')),
    updated_at integer,
    unique (upstream_id, repo, arch, filename),
    foreign key (upstream_id) references upstreams(id),
    foreign key (upstream_mirror_id) references upstream_mirrors(id)
) strict;
