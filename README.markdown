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
- Support other DBMS, for now only MySQL is supported. There should be a trait
and each DBMS should implement it (just an idea)
- Support different exports like `file -> SQL`, `SQL -> file`, `Monolog file ->
CSV file`, perhaps others


## Usage

For now there is only one possible usage.

```shell
./migration-log --input-module=monolog --input=the-log-file.log
```

### Options

The options depend on the modules you use. Please refer to the documentation of
the input and output modules to know all the possible options.
