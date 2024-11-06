# pacstash
Caching reverse proxy for arch-based distros.

## Warning: Alpha-quality software
Use at your own risk.

## Usage
- `sqlx database create` (Only first run)
- `sqlx migrate run`
- `go build`
- `DATABASE_URL=sqlite:dev.db WEB_PORT=3000 WEB_HOST=0.0.0.0 CACHE_ROOT=./fake_cache ./pacstash`
- Then, in each mirrorlist file under `/etc/pacman.d/` add an entry like the following:
- `CacheServer = http://pacstash-ip-or-url/u/$upstream-name/$repo/$arch`
- For cachyOS the repos are named as following:
    - `/etc/pacman.d/mirrorlist` -> arch
    - `/etc/pacman.d/cachyos-mirrorlist` -> cachyos
    - `/etc/pacman.d/cachyos-v3-mirrorlist` -> cachyosv3
        - Replace $arch with $arch_v3
    - `/etc/pacman.d/cachyos-v4-mirrorlist` -> cachyosv4
        - Replace $arch with $arch_v4

## Reasoning
If you have more than 1 host running an arch based os, you have to download each package you want to update once for each host. This costs you internet bandwidth and costs the mirror hosts cpu-time and bandwidth. By downloading each requested package only once, we save ourself and the mirror hosts time and money.

## Planned features
- [x] Basic caching
- [x] Mirror failover (If mirror1 fails, try mirror2 etc.)
- [ ] Statistics page (List number of packages, number of downloads etc.)
- [ ] Automatic cache expiry and deletion
- [ ] Version detection / keep only the most recent N versions of a package
- [ ] Configurable storage quotas per repo
- [ ] Dockerfile/Containerfile for containerized deployment
- [ ] Systemd service file for deployment as a SystemD service
- [ ] Support for other distros: Debian/Alpine
- [ ] Test with Arch, Manjaro, Endeavor etc. (Currently only tested with CachyOS)
- [ ] Automatically refresh the mirrorlist periodically
- [ ] Rank mirrors and keep statistics on which mirrors have the best success rate
- [ ] Interactive management page (Add/Modify/Del) Upstreams/Mirrors/Packages

## Libraries and Tools used
- go@1.23 (Pacstash binary)
    - fiber@v2 (Web framework)
    - github.com/glebarez/go-sqlite
- ruby@3.3 (Seed script)
- sqlite3 (Metadata database)
- sqlx-cli (Database migration management)

## Prior Art
This idea is not new. Here are some other examples of this being done:
- https://fernandobarillas.com/blog/2017/caching-pacman-packages-with-nginx/
    - Originally I used the method described in this blog post for caching. However I ran into the following problems:
        - Sometimes it would throw weird errors if the package wasn't found on a mirror.
        - NGINX wouldn't try more than one mirror (possibly I configured it incorrectly).
        - It needed to use mirrors that used exactly the same URL routing scheme after the hostname (unless I wanted to figure out how to properly use NGINX url rewriting).
        - Sometimes mirrors go away, or change hosting and become dramatically slower, in which case the nginx config would need rewritten.

- https://github.com/jaywilkas/pacyard
- https://help.ubuntu.com/community/Apt-Cacher%20NG

## Known issues
- Currently there is no cache expiry, packages will stay on disk and in the metadatadb until deleted manually.
- Need to randomly choose a mirror so it doesn't just hammer the first mirror in the db.
- File permissions could be stricter.
- If a file is deleted from the filesystem, but not from the metadatadb, attempting to fetch that package will fail indefinitely. Need to either remove the entry from metadata or flag it in some way.
- Failing to write download statistics will cause the request to fail.
- Need to set some sqlite `pragma`s.
- Requesting user agent is whatever `net/http`'s default is. It should report pacstash or pacman.
- Need to properly detect dev vs prod mode and setup fiber correctly.
- Needs request logging.

## Unknown issues
- Transactions? Do I need them for this?
- How do I only write statistics after the request has been sent? There should be some goroutine magic I can do.