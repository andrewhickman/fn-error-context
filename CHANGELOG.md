# Change Log

## 0.2.1

* Update syn to 2.0.4

## 0.2.0

* Fix some borrow checker errors with synchronous functions, and add the `#[context(move, "context")]` syntax to support functions which consume their arguments. Thanks to [@KaiJewson](https://github.com/KaiJewson) for [#5](https://github.com/andrewhickman/fn-error-context/pull/5).

## 0.1.2

* Fixed lints like `dead_code` not triggering for functions annotated with `#[context]` ([#3](https://github.com/andrewhickman/fn-error-context/issues/3)).

## 0.1.1

* Added async function support - thanks to [@tailhook](https://github.com/tailhook) for [#2](https://github.com/andrewhickman/fn-error-context/pull/2).
