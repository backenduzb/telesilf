# Telesilf

> The telegram based chat bot&usercontroller bot

---

Two variants are supported:

| Variant | Description |
|---------|-------------|
| **Variant 1** | Secretary mode can you give control in your chats for this bot |
| **Variant 2** | You can use manually without business connections |

---
## 🗂️ Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── README.md
└── src
    ├── config
    │   ├── mod.rs
    │   └── settings.rs
    ├── database
    ├── filters
    ├── handlers
    │   ├── business.rs
    │   ├── mod.rs
    │   └── start.rs
    ├── keyboards
    ├── main.rs
    ├── routes
    │   ├── mod.rs
    │   └── set.rs
    └── utils
        ├── mod.rs
        └── run.rs

9 directories, 14 files
```
---
## Overview

You already can start chat width `/start`.

### Integrations
| Categories | Skill | Description |
|--------|----------|-------------|
| `Secretary` | `with bussiness connection` | Can connect you profile without login |
| `Fully controls` | `with bussiness connection` | Can edit your own chats on secretary mode |


## 👨‍💻 Author

**Javohir** — [javohirdevp@gmail.com](mailto:javohirdevp@gmail.com)

---

> Built with using Rust + teloxide + telegram