# Monolog input module

[Monolog](https://seldaek.github.io/monolog/) is a logging library for PHP. All
the documentation you need to create Monolog logs are at the link, so I will not
describe here the strucure of a Monolog log.

One of the input modules this program can manage is Monolog, and as a module it
needs some arguments.

## Informations

- `module` The module has to be "monolog" in order to this module is
instanciated.
- `file` The file path where the logs are located. It's required to have the
read rights on this file.

## Usage

Here is an example of how to use the Monolog module.

```bash
./migrate-log --input-module=monolog --input-file=/home/user/application.log \
--output-module=mysql
```

> Note that the output module choosen here is only an example, you can use the
> module that you want.

## TODO

- Parse JSON in `extra_raw` and `context_raw` in order to fill `extra` and
`context`
