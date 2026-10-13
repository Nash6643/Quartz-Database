# QuartzDB

**A Relational Database Management System Built from Scratch in Rust**

QuartzDB is a personal systems programming project that explores the internal architecture of relational database management systems through a ground-up implementation in Rust.

Rather than relying on an existing database engine, QuartzDB implements its own SQL-inspired query language, lexer, recursive-descent parser, abstract syntax tree, query planner, execution engine, transaction handling, indexing, and page-based storage system.

This project focuses on understanding how database engines process queries, enforce relational constraints, manage data, and persist records to disk.

## Overview

QuartzDB is also designed as a learning-oriented relational database engine that brings together several fundamental database concepts in a single Rust application.

- **Custom query engine:** Lexing, parsing, AST construction, query planning, and execution.
- **Relational data management:** Tables, schemas, data types, and CRUD operations.
- **Constraint enforcement:** Primary keys, unique constraints, foreign keys, and cascading deletes.
- **Query capabilities:** Filtering, projections, aggregate functions, grouping, and joins.
- **Transaction handling:** Transaction commands and rollback support for failed operations.
- **Storage architecture:** A pager, fixed-size pages, catalog, buffer cache, serialization, and disk persistence.
- **Hash-based indexing:** Index management with automatic rebuilding.

> QuartzDB is an educational database implementation, not a production-ready replacement for PostgreSQL, SQLite, or MySQL.

---

## Features

### 1. Custom Query Language

QuartzDB uses a custom SQL-inspired language designed to demonstrate how database commands are translated into executable operations.

The query processing pipeline consists of:

1. **Lexer** — Converts input into tokens.
2. **Parser** — Validates syntax and constructs an abstract syntax tree.
3. **AST** — Represents queries in a structured form.
4. **Query planner** — Prepares parsed queries for execution.
5. **Query executor** — Executes operations against the database.
6. **Storage layer** — Manages catalog information, pages, and persistence.

Supported commands include:

| Command | Purpose |
|---|---|
| `MAKE` | Create a table |
| `ADD` | Insert records |
| `SHOW` | Display tables or selected columns |
| `FIND` | Filter records using conditions |
| `CHANGE` | Update existing records |
| `REMOVE` | Delete records |
| `COUNT` | Count records |
| `SUM` | Calculate a sum |
| `AVG` | Calculate an average |
| `MIN` / `MAX` | Find minimum and maximum values |
| `GROUP` | Group records for aggregation |
| `JOIN` | Join two tables |
| `SAVE` / `LOAD` | Save and load database state |
| `HELP` / `EXIT` | Display help or exit the application |

Transaction commands include `BEGIN`, `COMMIT`, and `ROLLBACK`.

### 2. Schema and Table Management

QuartzDB supports table creation, schema validation, and relational constraints.

Supported data types:

- `int` — Integer values
- `text` — Text values
- `bool` — Boolean values

Supported constraints:

- Primary keys
- Unique constraints
- Nullable and non-null columns
- Foreign keys
- Cascading deletes

Example:

```sql
MAKE users
id:int:pk
name:text
age:int
active:bool
```

This defines a `users` table with an integer primary key, a text column, an integer age, and a Boolean status.

### 3. CRUD Operations

QuartzDB implements the fundamental operations required to manage relational data.

**Insert records**

```sql
ADD users
1 John 20 true
```

**Retrieve records**

```sql
SHOW users
```

**Select specific columns**

```sql
SHOW users name age
```

**Filter records**

```sql
FIND users IF age > 18
```

Supported comparison and logical operators include:

- `=`, `!=`
- `>`, `<`, `>=`, `<=`
- `LIKE`
- `AND`, `OR`

**Update records**

```sql
CHANGE users
SET age 21
IF id = 1
```

**Delete records**

```sql
REMOVE users
IF id = 1
```

### 4. Aggregate Functions and Grouping

QuartzDB supports aggregate operations for summarizing table data.

```sql
COUNT users

SUM users salary

AVG users salary

MIN users age

MAX users age
```

Grouping allows records to be organized by a specified field before applying an aggregate operation.

```sql
GROUP employees department COUNT
```

This demonstrates grouping employee records by department and counting records in each group.

### 5. Relational Joins

QuartzDB supports joining two tables through a matching-column condition.

```sql
JOIN students
ON students.class_id = classes.id
```

This enables queries to combine related records across tables.

### 6. Transactions and Rollback

QuartzDB includes a basic transaction interface with the following commands:

```text
BEGIN
COMMIT
ROLLBACK
```

The current implementation includes rollback support for failed operations. Full ACID guarantees, concurrent transaction isolation, and crash-safe recovery are not yet part of the stated implementation.

### 7. Referential Integrity

The database validates relational constraints to help maintain consistent data.

Implemented capabilities include:

- Primary key uniqueness
- Unique value enforcement
- Foreign key validation
- Cascading deletes

These mechanisms demonstrate how a relational engine protects relationships between tables during data modifications.

### 8. Hash-Based Indexing

QuartzDB implements hash-based indexes to support indexed data access.

Current capabilities:

- Hash-based indexing
- Automatic index rebuilding

**Planned:** B+ tree indexing for more advanced lookup and range-query strategies.

### 9. Page-Based Storage Engine

QuartzDB is evolving beyond an in-memory database toward a disk-backed, page-oriented storage architecture.

Implemented storage components include:

- Pager
- Fixed-size pages of 4,096 bytes
- Page allocation
- Buffer cache
- Catalog
- Page serialization
- Disk persistence

The storage layer serializes rows before writing them into database pages.

```text
              Database
                  |
                  v
                Pager
                  |
                  v
                 Page
                  |
                  v
              quartz.db
```

The page-based design provides a foundation for future improvements, including multi-page tables, more efficient storage layouts, and advanced recovery mechanisms.

---

## Architecture

QuartzDB separates query processing from database management and storage responsibilities.

```text
                 User
                  |
                  v
              REPL Console
                  |
                  v
                 Lexer
                  |
                  v
                Parser
                  |
                  v
             Abstract Syntax Tree
                  |
                  v
             Query Planner
                  |
                  v
             Query Executor
                  |
                  v
               Database
                  |
          +-------+-------+
          |               |
          v               v
        Catalog          Pager
                           |
                           v
                     Database Pages
                           |
                           v
                       quartz.db
```

### Architectural Components

| Component | Responsibility |
|---|---|
| REPL Console | Accepts commands and presents results |
| Lexer | Tokenizes query input |
| Parser | Parses tokens and constructs query representations |
| AST | Represents query structure |
| Query Planner | Prepares queries for execution |
| Query Executor | Executes database operations |
| Database | Coordinates tables, records, and constraints |
| Catalog | Manages database metadata |
| Pager | Coordinates page access and persistence |
| Page | Represents a fixed-size storage unit |

This modular organization keeps query processing, relational logic, and storage responsibilities distinct.

---

## Example Session

Create a table and insert records:

```sql
MAKE users
id:int:pk
name:text
age:int
```

```sql
ADD users
1 John 20
```

```sql
ADD users
2 Alice 18
```

Display the table:

```sql
SHOW users
```

Example output:

```text
id    name    age

1     John    20
2     Alice   18
```

Filter records:

```sql
FIND users IF age > 18
```

The query returns records matching the specified condition.

---

## Project Structure

```text
QuartzDB/
├── src/
│   ├── database.rs
│   ├── page.rs
│   ├── pager.rs
│   ├── catalog.rs
│   ├── query/
│   │   ├── lexer.rs
│   │   ├── parser.rs
│   │   ├── token.rs
│   │   ├── ast.rs
│   │   ├── executor.rs
│   │   ├── planner.rs
│   │   └── helpers.rs
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

The `query` module contains the language-processing and execution components, while the database, catalog, page, and pager modules implement core data-management and storage functionality.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo, included with a standard Rust installation
- Git

### Installation

Clone the repository:

```bash
git clone https://github.com/<your-username>/QuartzDB.git
cd QuartzDB
```

Build the project:

```bash
cargo build
```

Run the application:

```bash
cargo run
```

Run the test suite, if tests are configured:

```bash
cargo test
```

> Replace `<your-username>` with your GitHub username. The commands above assume the repository uses the standard Cargo project layout.

---

## Technologies

| Technology | Role |
|---|---|
| Rust | Core implementation and systems programming |
| Cargo | Build and dependency management |
| Serde | Serialization and deserialization |
| Serde JSON | JSON serialization support |

---

## Engineering Concepts

QuartzDB explores the implementation of several fundamental database and systems programming concepts:

- Lexical analysis and recursive-descent parsing
- Abstract syntax trees
- Query planning and execution
- Relational schemas and constraint validation
- CRUD operations and conditional filtering
- Aggregate functions and grouping
- Relational joins
- Transactions and rollback
- Referential integrity
- Hash-based indexing
- Serialization and disk persistence
- Page allocation and buffer management
- Separation of query processing and storage layers

---

## Roadmap

### Version 1.1 — Storage Improvements

- [ ] Improve page storage and serialization
- [ ] Support multi-page tables
- [ ] Optimize storage layouts
- [ ] Improve persistence and page management

### Version 2.0 — Client-Server Architecture

- [ ] Implement a TCP database server
- [ ] Support multiple clients
- [ ] Develop a client API
- [ ] Add authentication

### Version 3.0 — Query Optimization

- [ ] Implement B+ tree indexes
- [ ] Introduce query optimization
- [ ] Collect table and index statistics
- [ ] Develop a cost-based execution planner

### Version 4.0 — Advanced Transactions

- [ ] Implement Write-Ahead Logging (WAL)
- [ ] Add crash recovery
- [ ] Explore Multi-Version Concurrency Control (MVCC)
- [ ] Support concurrent transactions

---

## Motivation

QuartzDB was created to develop a deeper understanding of how relational database management systems work internally.

Building a database from scratch requires reasoning about how text becomes executable queries, how records are validated, how related tables remain consistent, and how data moves from memory into persistent storage.

By implementing these components directly in Rust, QuartzDB serves as a practical exploration of database internals, systems programming, and software architecture.

The project also provides a foundation for experimenting with more advanced database features, from tree-based indexes and query optimization to transaction recovery and client-server communication.

## Author

**Omar Nashiru-Deen**

Software Engineering Student  
Istanbul Ticaret University

- GitHub: [@your-username](https://github.com/<your-username>)

## License

QuartzDB is distributed under the MIT License. See the `LICENSE` file for details.
