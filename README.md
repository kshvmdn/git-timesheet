## git-timesheet

> Compute detailed commit stats for local git repositories.

Built as a means of learning Rust, would greatly appreciate any and all feedback.

### Contents

- [Demo](#demo)
- [Installation](#installation)
- [Usage](#usage)
- [Contribute](#contribute)
- [Credit](#credit)
- [License](#license)

### Demo

_Coming soon._

### Installation

You should have [Rust](https://www.rust-lang.org/en-US/) and [Cargo](https://crates.io/) installed.

- Install with Cargo (`~/.cargo/bin` should be in your `$PATH`):

  ```sh
  $ cargo install --git https://github.com/kshvmdn/git-timesheet
  $ which git-timesheet
  ~/.cargo/bin/git-timesheet
  ```

- Or, build manually:

  ```sh
  $ git clone https://github.com/kshvmdn/git-timesheet.git git-timesheet && cd $_
  $ cargo build --release
  $ mv target/release/git-timesheet /usr/local/bin # or any other directory in your $PATH!
  $ which git-timesheet
  /usr/local/bin/git-timesheet
  ```

### Usage

git-timesheet works just like any other git subcommand. View the [demo](#demo) for sample output.

```sh
$ cd /path/to/git/directory
$ git timesheet
...
```

### Contribute

This project is completely open source, feel free to open an issue or submit a pull request. Very new to Rust, would love to hear any feedback.

#### TODO

- Calculate average commit rate (per hr, per day, etc) and other stats.
- Add functionality to filter committers (by email).
- Improve error handling.

### Credits

- Inspired by [wolever/git-work-life](https://gist.github.com/wolever/2a15db70f8cb255d753b2cdbb8a718ce).

### License

git-timesheet is released under the [MIT license](./LICENSE).
