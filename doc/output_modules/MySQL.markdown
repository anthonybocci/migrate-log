# MySQL output module

MySQL is a relational database management system. This documentation doesn't aim
to explain what is really MySQL nor even SQL, please refer to their own
documentation for that. So, I will only explain what this module can do.

One of the output modules this program can manage is MySQL, and as a module it
needs some arguments.

## Informations

- `module` The module has to be "mysql" in order to this module is
instanciated.
- `file` (optional) The file path where the exported logs should be written. If
given, the write rights are required on the file. If not given, the result is
print onto STDOUT.

## Usage

Here is an example of how to use the MySQL module.

```bash
./migrate-log --input-module=monolog --input-file=/home/user/application.log \
--output-module=mysql [--output-file=/home/user/application.sql]
```

> Note that the "--input-" options depend on the input module you've choosed,
> here Monolog is used only as example, you can use the one you prefer.

> The argument that is between `[` and `]` is optional.
