# Migrate log

Migrate-log is a program that perform a log migration.

This project has two main goals:
- The first one is to practice Rust that is a pretty new language for me.
- The second one is to provide a program that perform a migration of logs from
file to database or other.

## Work in-progress

This project is at its beginning and there are lot of thing that can be done.

### Done

- Reads a log file
- Loops over log lines ignoring the mis-formatted ones
- Instanciates `Log`s for each line
- Displays on STDOUT the SQL statements corresponding to each Log

### Todo

- Check if the log file exists
- Allows different format of logs. For now only Monolog is allowed. There should
be a trait and Monolog should implement it (just an idea)
- Support other DBMS, for now only MySQL is supported. There should be a trait
and each DBMS should implement it (just an idea)
- Support different exports like `file -> SQL`, `SQL -> file`, perhaps others


## Usage

For now there is only one possible usage.

```shell
./migration-log --input=the-log-file.log
```

### Options

There is the only available option:

- `--input`: The path to the log file to convert to SQL.
