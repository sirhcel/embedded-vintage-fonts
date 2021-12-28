# Embedded Vintage Fonts

Your dearly beloved fonts from
[embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) 0.6
back for the overhauled font handling from 0.7.

They are back all but one: `Font6x6` uses variable-width glyphs which are
currently not supported by 0.7.

## Specimens

### `FONT_6X8` formerly known as `Font6x8`

![FONT\_6X6](https://raw.githubusercontent.com/sirhcel/embedded-vintage-fonts/master/data/font6x8.png)

### `FONT_6X12` formerly known as `Font6x12`

![FONT\_6X12](https://raw.githubusercontent.com/sirhcel/embedded-vintage-fonts/master/data/font6x12.png)

### `FONT_8X16` formerly known as `Font6x16`

![FONT\_8X16](https://raw.githubusercontent.com/sirhcel/embedded-vintage-fonts/master/data/font8x16.png)

### `FONT_12X16` formerly known as `Font12x16`

![FONT\_12X16](https://raw.githubusercontent.com/sirhcel/embedded-vintage-fonts/master/data/font12x16.png)

### `FONT_24X32` formerly known as `Font24x32`

An upscaled version of `FONT_12X16`.


## Examples

Have a look at the fonts with an adapted variant of
[ProFont](https://github.com/wezm/profont)'s font debugger using the
`embedded-graphics` simulator:
```
$ cargo run --example debugger
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your discretion.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
