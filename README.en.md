# transform_specific_chars.anm2

[English](README.en.md) | [日本語](README.md)

[![AviUtl2 Catalog](https://aviutl2-catalog-badge.sevenc7c.workers.dev/badge/v/sevenc-nanashi.transform_specific_chars-anm2)](https://aviutl2-catalog-badge.sevenc7c.workers.dev/package/sevenc-nanashi.transform_specific_chars-anm2)

A script for AviUtl2 that applies transforms only to specified characters in a text object.
This script requires the text option "per-character individual object" to be enabled.

## Installation

Download `sevenc-nanashi.transform_specific_chars-anm2-v{{version}}.au2pkg.zip` from [Releases](https://github.com/sevenc-nanashi/transform_specific_chars.anm2/releases/latest), then drag and drop it into the AviUtl2 preview.

## Usage

1. Enable "per-character individual object" in your text object.
2. Add this script.
3. Set `Target Chars`.
4. Toggle `Regex` if needed.
5. Adjust position, rotation, scale, transparency, and color parameters.

## Target Pattern Syntax

When `Regex` is off, `Target Chars` is parsed with the following rules:

- Plain characters: match the character itself.
- `[[A-Z]]`: match a character range.
- `{{Hiragana}}`: match a Unicode property (Corresponds to `\p{Hiragana}` in [regex](https://docs.rs/regex)).

When `Regex` is on, `Target Chars` is evaluated as a Rust [regex](https://docs.rs/regex), and the transform is applied to characters within the matched range.

## Notes

- Text containing the clear notation `<c...>` and script notation (`<?...?>`) will cause an error.

## PI

Like [@sigma-axis](https://github.com/sigma-axis)'s scripts, this script supports PI (Parameter Injection).\
You can set parameters using Lua expressions.\
Values set by PI take precedence over trackbar values.

In most cases, you do not need PI, but it allows more flexible configuration when needed.

### Available Keys

- `target_chars` (`string`): Target Chars
- `invert_target` (`boolean`): Invert Target Characters
- `regex` (`boolean`): Use Regex
- `dx`, `dy`, `dz` (`number`): Position offset
- `center_x`, `center_y`, `center_z` (`number`): Center
- `angle_x`, `angle_y`, `angle_z` (`number`): Rotation
- `zoom`, `scale_x`, `scale_y`, `scale_z` (`number`): Scale (`1.0` is default size)
- `transparency` (`number`): Transparency (`0.0` is opaque, `1.0` is fully transparent)
- `color` (`number | false`): Text Color
- `terminate` (`boolean`): Terminate Effect
- `debug` (`boolean`): Debug Mode

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
