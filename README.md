# peq-takp-pop-merge

Project using Rust to merge takp content into peq up until pop era.

## Pre-requisites

-   MySql or MariaDB instance running, preloaded with PEQ data and TAKP data.
-   Environment variables set with URLs for PEQ and TAKP databases:
    -   PEQ_DATABASE_URL
    -   TAKP_DATABASE_URL
    -   E.g. `PEQ_DATABASE_URL=mysql://<user>:<password>@<host>/peq`

## Note to self

Read the docs: https://diesel.rs/guides/schema-in-depth.html
