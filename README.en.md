# disassembler.anm2

[English](README.en.md) | [日本語](README.md)

[![AviUtl2 Catalog](https://aviutl2-catalog-badge.sevenc7c.workers.dev/badge/v/sevenc-nanashi.disassembler-anm2)](https://aviutl2-catalog-badge.sevenc7c.workers.dev/package/sevenc-nanashi.disassembler-anm2)

A script for AviUtl2 that splits an image into parts and turns them into individual objects.

## Installation

Download `sevenc-nanashi.disassembler-anm2-v{{version}}.au2pkg.zip` from [Releases](https://github.com/sevenc-nanashi/disassembler.anm2/releases/latest), then drag and drop it into the AviUtl2 preview.

## PI

Like [@sigma-axis](https://github.com/sigma-axis)'s scripts, this script supports PI (Parameter Injection).\
You can set parameters using Lua expressions.\
Values set by PI take precedence over trackbar values.

In most cases, you do not need PI, but it allows more flexible configuration when needed.

### Available Keys

- `threshold` (`number`): Alpha threshold (`0.0` to `1.0`)
- `sort_mode` (`number`): Sort direction (`0` to `15`)
- `reference_point` (`number`): Reference point (`0` to `8`)
- `quantize_x` (`number`): X quantization (`>= 1`)
- `quantize_y` (`number`): Y quantization (`>= 1`)
- `quantize_shift_x` (`number`): X quantization shift
- `quantize_shift_y` (`number`): Y quantization shift
- `show_quantize_grid` (`boolean`): Visualize split parts
- `debug` (`boolean`): Debug mode

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
