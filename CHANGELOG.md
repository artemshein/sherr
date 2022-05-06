0.5.1
=====

* Include only file name, but not path in diag_position

0.5.0
=====

* Removed throw macro
* Fix more imports for anyhow

0.4.3
=====

* Fix imports for anyhow

0.4.2
=====

* Switch edition to 2021

0.4.1
=====

* Fixed import issue

0.4.0
=====

* Added throw!
* Add implicit imports

0.3.1
=====

* Added bail_diag!

0.3.0
=====

* Switch to anyhow from failure
* Remove fail feature
* Major API overhaul

0.2.10
======

* Fix is_a_tty check on Windows

0.2.9
=====

* Activate colored logging when it is a terminal

0.2.8
=====

* Switch to new `use macro` syntax

0.2.7
=====

* Fixed waring message formatting

0.2.6
=====

* Added stdout_dispatch_with_target
* Output position on diag_err
* Output stack trace on diagnostics errors

0.2.2-0.2.5
===========

* Fixed macro name resolution

0.2.1
=====

* Added diag_err! macro

0.2.0
=====

* Added fail feature for failure support.
* Fixed diagnostics base log level.
* Added optional path parameter for init_logger.

0.1.0
=====

Initial release.