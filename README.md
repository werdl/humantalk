## humantalk
> human friendly debug messages
## why
- not all your end users will be developers
- not all your end users will be familiar with your codebase
- so why not make it easier for them to understand what's going on?
- streamlines bug reporting, highly customisable
## what
- has the ability to produce warnings, info messages, debug messages (if debug symbols are enabled), and non-fatal messages, which are displayed in various colors
- can do fatal errors, which will also generate a crash file (see an example in [crash_report.log](crash_report.log)), and instruct the user to submit a bug report on your docsite/github issues/github discussion/email etc.