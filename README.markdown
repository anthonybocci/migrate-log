# Migrate log

Migrate-log is a program that perform a log migration.

This project has two main goals:
- The first one is to practice Rust that is a pretty new language for me.
- The second one is to provide a program that perform a migration of logs from
file to database or other.

### Features

- Uses an extensible module system to import the logs
- Imports the logs to get a vector of logs
- Uses an extensible module system to export the logs
- Exports the logs an any format according to the choosen output module

## Usage

See the [Usage documentation
file](https://github.com/anthonybocci/migrate-log/tree/master/doc/usage.markdown)
to see how to use this program.

### Options

The options depend on the modules you use. Please refer to the documentation of
the input and output modules to know all the possible options. All
documentations are in the `doc/` directory.
