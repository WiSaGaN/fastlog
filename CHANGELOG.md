# Changelog

This project roughly adheres to [Semantic Versioning](http://semver.org/). For 0.x.y releases, `x` is the major version in semver, while `y` is the minor version.

## 0.2.1 - 2019-11-26
* Implement `Default` for `LogBuilder`
* Upgrade to 2018 edition and clean up warnings

## 0.2.0 - 2019-02-21

* Upgrade to support `log` 0.4
* Change to use `crossbeam_channel::bounded` as log queue
* Bump minimum version to rustc 1.32.0

## 0.1.1 - 2016-07-01

* Change to use `std::sync::mpsc::sync_channel` as log queue
* Add inline to improve LTO

## 0.1.0 - 2016-05-08

First release
