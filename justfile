reset:
    rm dev.db
    rm -rf ./fake_cache
    sqlx database create
    sqlx migrate run
    ruby seed.rb