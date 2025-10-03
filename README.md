1. Add a .env at the root of the infra project with DATABASE_URL=postgres://usr:pwd@localhost:5432/name-postgres changing for the name of your posgres database and usr/pwd.
2. cargo build
3. cargo run --bin write_objects_from_xlsx

## License
This work is dual-licensed under the MIT License and the GPL 2.0 (or any later version).
`SPDX-License-Identifier: MIT OR GPL-2.0-or-later`
``` [12, 8]
