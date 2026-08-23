# SimpleBackdoor

> The backdoor virus for remote shell scripts

---

Three variants are supported:

| Variant | Description |
|---------|-------------|
| **Admin** | Admin can control clients terminal and processes |
| **Client** | Can't do not thing  |
| **Server** | For global connections  |

---
## 🗂️ Project Structure

```
.
.
├── admin
│   ├── mobile
│   └── rust
│       ├── Cargo.lock
│       ├── Cargo.toml
│       ├── .gitignore
│       └── src
│           ├── connection.rs
│           ├── main.rs
│           └── protocol.rs
├── client
│   └── rust
│       ├── Cargo.lock
│       ├── Cargo.toml
│       ├── .gitignore
│       └── src
│           ├── connection.rs
│           ├── executor.rs
│           ├── main.rs
│           └── protocol.rs
├── README.md
└── server
    ├── Cargo.lock
    ├── Cargo.toml
    ├── Dockerfile
    ├── .dockerignore
    ├── .env
    ├── .env.example
    ├── .gitignore
    └── src
        ├── main.rs
        ├── protocol.rs
        ├── state.rs
        ├── tcp.rs
        └── .udp
            ├── main.rs
            ├── protocol.rs
            ├── state.rs
            └── udp.rs

11 directories, 29 files
```
---
## Importand 
Use this for only joke

## 👨‍💻 Author
**Javohir** — [javohirdevp@gmail.com](mailto:javohirdevp@gmail.com)

---

> Built with using Rust + teloxide + telegram