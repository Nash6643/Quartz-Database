# QuartzDB

> A relational database management system built completely from scratch in **Rust**.

QuartzDB is a personal systems programming project that explores how modern relational database management systems work internally. Instead of relying on existing database libraries, QuartzDB implements its own SQL-like language, query parser, execution engine, transaction system, indexing, and page-based storage architecture.

The goal of the project is to understand the internals of databases such as PostgreSQL, SQLite and MySQL by rebuilding their core concepts from first principles.

---

# Features

## Query Language

- SQL-inspired query syntax
- Custom Lexer
- Recursive-descent Parser
- Abstract Syntax Tree (AST)
- Query Planner
- Query Executor

Supported commands include:

- MAKE TABLE
- ADD
- SHOW
- FIND
- CHANGE
- REMOVE
- COUNT
- SUM
- AVG
- MIN
- MAX
- GROUP
- JOIN
- SAVE
- LOAD
- HELP
- EXIT

---

# Database Features

## Table Management

- Create tables
- Multiple column data types
- Schema validation
- Primary Keys
- Unique Constraints
- Nullable / Non-null columns
- Foreign Keys

Example

```sql
MAKE users
id:int:pk
name:text
age:int
active:bool
```

---

## CRUD Operations

### Insert

```sql
ADD users
1 John 20 true
```

### Read

```sql
SHOW users
```

### Projection

```sql
SHOW users name age
```

### Conditional Search

```sql
FIND users IF age > 18
```

Supports

- =
- !=
- >
- <
- >=
- <=
- LIKE
- AND
- OR

---

## Update

```sql
CHANGE users
SET age 21
IF id = 1
```

---

## Delete

```sql
REMOVE users
IF id = 1
```

---

# Aggregate Functions

QuartzDB currently supports

```sql
COUNT users

SUM users salary

AVG users salary

MIN users age

MAX users age
```

---

# GROUP BY

Grouping with aggregate operations

Example

```sql
GROUP employees department COUNT
```

---

# JOIN

Supports joining two tables.

Example

```sql
JOIN students
ON students.class_id = classes.id
```

---

# Transactions

QuartzDB includes a simple transaction engine.

Supported operations

- BEGIN
- COMMIT
- ROLLBACK

Current implementation supports rollback of failed operations.

---

# Referential Integrity

QuartzDB enforces relational constraints.

Implemented

- Primary Keys
- Unique Constraints
- Foreign Keys
- Cascading Deletes

---

# Indexing

Current implementation

- Hash-based indexes
- Automatic index rebuilding

Future

- B+ Tree indexes

---

# Storage Engine

QuartzDB is transitioning from an in-memory database to a page-based storage engine.

Implemented

- Pager
- Fixed-size pages (4096 bytes)
- Page allocation
- Buffer cache
- Catalog
- Page serialization
- Disk persistence

Architecture

```
Database
     │
     ▼
Pager
     │
     ▼
Page
     │
     ▼
quartz.db
```

Rows are serialized before being written into database pages.

---

# Internal Architecture

```
                User

                  │

                  ▼

              REPL Console

                  │

                  ▼

               Lexer

                  │

                  ▼

               Parser

                  │

                  ▼

             AST (Query)

                  │

                  ▼

             Query Planner

                  │

                  ▼

             Query Executor

                  │

                  ▼

               Database

                  │

        ┌─────────┴─────────┐
        ▼                   ▼

     Catalog             Pager

                              │

                              ▼

                          Database Pages

                              │

                              ▼

                          quartz.db
```

---

# Project Structure

```
src/

database.rs
page.rs
pager.rs
catalog.rs

query/
    lexer.rs
    parser.rs
    token.rs
    ast.rs
    executor.rs
    planner.rs
    helpers.rs

main.rs
```

---

# Data Types

QuartzDB currently supports

- Integer
- Text
- Boolean

---

# Error Handling

The engine validates

- Invalid table names
- Invalid columns
- Type mismatches
- Duplicate primary keys
- Duplicate unique values
- Missing foreign keys
- Invalid queries
- Invalid syntax

---

# Example Session

```text
MAKE users
id:int:pk
name:text
age:int

ADD users
1 John 20

ADD users
2 Alice 18

SHOW users
```

Output

```text
id    name    age

1     John    20
2     Alice   18
```

---

# Technologies

- Rust
- Cargo
- Serde
- Serde JSON

---

# Concepts Implemented

This project explores many database internals including

- Lexical Analysis
- Parsing
- Abstract Syntax Trees
- Query Planning
- Query Execution
- Schema Validation
- Transactions
- Rollback
- Referential Integrity
- Hash Indexes
- Serialization
- Page-based Storage
- Buffer Management

---

# Current Status

Current Version

```
QuartzDB v1.0
```

Completed

- SQL-like parser
- Query execution engine
- CRUD operations
- Projection
- Filtering
- Aggregate functions
- GROUP BY
- JOIN
- Transactions
- Rollback
- Foreign Keys
- Cascading Deletes
- Hash Indexes
- Pager
- Catalog
- Page-based Storage Engine

---

# Roadmap

## Version 1.1

- Improve page storage
- Multi-page tables
- Better serialization
- Storage optimisation

## Version 2.0

- TCP Database Server
- Multi-client support
- Client API
- Authentication

## Version 3.0

- B+ Tree indexes
- Query Optimiser
- Statistics
- Cost-based execution planner

## Version 4.0

- Write Ahead Logging (WAL)
- Crash Recovery
- MVCC
- Concurrent Transactions

---

# Motivation

QuartzDB was built as a systems programming project to better understand how modern relational databases are implemented internally.

Rather than using existing database libraries, every major component—including parsing, execution, storage, transactions, indexing, and persistence—is implemented manually in Rust as a learning exercise and portfolio project.

---

# Author

**Omar Nashiru-Deen**

Software Engineering Student  
Istanbul Ticaret University

GitHub:
https://github.com/<Nash6643>

---

# License

MIT License