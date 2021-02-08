# A Passive DNS database implemented in Rust

![Build](https://github.com/lberezy/pdns-rust/workflows/Rust/badge.svg)

---

TODO:

- [ ] Add DNS traffic capture capability
    - [ ] Use eBPF/XDP for high-performance capture
    - [ ] Benchmark current database/index layer (`sled`/`tantivy` via `pallet`)
- [ ] Add GraphQL query layer
    - [ ] Add mutations for adding new entries
    - [ ] Add subscription to watch incoming new resolutions
- [ ] Add embedded web interface