# 📦 oxidoc

**oxidoc** is a tiny, experimental, intentionally-rusty **Document Database** written in Rust.  
This project is mostly a **devlog**, a learning journey into storage engines, LSM-trees, binary formats, database internals, client/server architectures... 
and all the fun (and pain) behind building a database from scratch.

It’s not meant to be production-ready. 

---

## 🧠 Motivation

I’ve worked with NoSQL systems for years (MongoDB, HBase, etc.), but I always felt there was something missing: I *used* these systems, but I didn’t fully *understand* their internals.

Concepts like:

- LSM-trees
- WALs
- SSTables
- replica protocols
- storage layouts
- indexing strategies

…are clear in theory, but they only become *real* when you try to implement them yourself.

At the same time, I’ve wanted to learn Rust deeply, and following generic tutorials bores me to death.

So I merged the two needs, and **oxidoc** was born.

---

## 🎯 Project Goals

This project aims to explore database internals through hands-on implementation.
The primary goal is to learn by building something real.

The roadmap includes:

- a simple **in-memory document store**
- a **client/server architecture**
- a minimal request/response **wire protocol**
- durability via a **Write-Ahead Log**
- a **Memtable + SSTable** LSM-style storage engine
- examples, devlogs, and transparent documentation

If it becomes a (working) toy database, great.
If not, at least I’ll have learned a lot along the way.
---

## 🏗️ Project Structure

This repository is organized as a **Rust workspace**:

```
oxidoc/
│
├─ oxidoc-core/ # storage engine, documents, collections, database logic
├─ oxidoc-server/ # TCP server, request router, protocol handling
└─ oxidoc-cli/ # client CLI to talk to the server
```


### **oxidoc-core**
Provides the internal mechanics:

- JSON-based `Document` type
- `Collection` (key → document map)
- `Database` (collection manager)
- future storage modules (`wal`, `sstable`, `lsm_tree`, …)

### **oxidoc-server**
Implements:
- the TCP listener
- per-connection handler
- minimal protocol parsing
- routing commands to the core engine

---

## 🚀 Getting Started

### Build the entire workspace

```bash
cargo build --workspace
```

### Run the server

```bash
cargo run -p oxidoc-server
```

### Connect via netcat (for now)

```bash
nc localhost 7878
```

## 🤝 Contributions
This project is mostly a personal playground,
but PRs, discussions and refactors are welcome.