# newline-converter
`newline-converter` is a simple library used for converting the newline characters in strings between Windows `\r\n` and Unix `\n` style. It mainly serves as a backend for [Rust Newline converter](https://github.com/spitfire05/rnc) CLI tool.

[![Build Status](https://dev.azure.com/michal0805/rnc/_apis/build/status/spitfire05.rnc?branchName=master)](https://dev.azure.com/michal0805/rnc/_build/latest?definitionId=1&branchName=master) ![Crates.io](https://img.shields.io/crates/v/newline-converter)

This lib has two significant advantages over using `string.replace`:
* Looks for correct linebreaks on source platform. For example, lone `\r` characters won't get replaced by `dos2unix`  call:
  ```rust
  use newline_converter::dos2unix;
  assert_eq!(
    dos2unix("\nfoo\rbar\r\n"),
    "\nfoo\rbar\n"
  );
  ```
* Being significantly faster (about two times in normal circumstances, and about twenty times faster when the input is already in correct format).
