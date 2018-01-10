# Usage

This documentation aims to explain how to use Migration Log.
It will not list all possibilities for every input or output module, but it will
show the principles of how this program can be used to migrate your logs.

## Principles

There are two simple principles in this program and they are present as modules,
input and output.

The explanation is easy, your logs can be in any format and you want to export
them in an other format of your choice. So, the usage of this program is guided
by these two principles the input module and the output module.

The program arguments that can be given to the program may vary according to
which modules you use. Some modules take only one argument, some others may take
many.

## Base

The base that is required is to set an input and an output module. To do so, the
`--input-module` and `--output-module` are required.

```bash
./migrate-log --input-module=... --output-module=...
```

> Of course, "..." here should be replaced by the module name.

You can see the existing
[input](https://github.com/anthonybocci/migrate-log/tree/master/doc/input_modules) of
[output](https://github.com/anthonybocci/migrate-log/tree/master/doc/output_modules)
modules in the documentation.
Each module explains in its documentation what argument may be given.
