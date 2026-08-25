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

### Tug'ilgan kun tabrigi 🎂

Botni guruhga qo'shib, `/birthday_for Ism` buyrug'ini yozing — bot o'sha ism
uchun katta ASCII harflardan yasalgan, HTML `marquee` kabi harakatlanuvchi
bezakli tabrikni yuborib, uni yangilab turadi. To'xtatish uchun
`/stopbirthday` yozing (yoki boshqa `/birthday_for` — u avvalgisini
almashtiradi).

| Buyruq | Tavsif |
|--------|--------|
| `/birthday_for Ism` | ASCII-marquee tabrikni boshlaydi (alias: `/birdthday_for`) |
| `/stopbirthday` | Faol tabrikni to'xtatadi (alias: `/stopbirdthday`) |

### Integrations
| Categories | Skill | Description |
|--------|----------|-------------|
| `Secretary` | `with bussiness connection` | Can connect you profile without login |
| `Fully controls` | `with bussiness connection` | Can edit your own chats on secretary mode |


## 👨‍💻 Author

**Javohir** — [javohirdevp@gmail.com](mailto:javohirdevp@gmail.com)

---

> Built with using Rust + teloxide + telegram