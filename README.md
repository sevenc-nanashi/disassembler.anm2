# transform_specific_chars.anm2

[English](README.en.md) | [日本語](README.md)

[![AviUtl2 Catalog](https://aviutl2-catalog-badge.sevenc7c.workers.dev/badge/v/sevenc-nanashi.transform_specific_chars-anm2)](https://aviutl2-catalog-badge.sevenc7c.workers.dev/package/sevenc-nanashi.transform_specific_chars-anm2)

指定した文字のみに加工を適用する AviUtl2 スクリプト。

## インストール

[Releases](https://github.com/sevenc-nanashi/transform_specific_chars.anm2/releases/latest) から `sevenc-nanashi.transform_specific_chars-anm2-v{{version}}.au2pkg.zip` をダウンロードし、AviUtl2 のプレビューにドラッグ＆ドロップしてください。

## 使い方

1. テキストオブジェクトで「文字毎に個別オブジェクト」を有効にします。
2. このスクリプトを追加します。
3. `対象文字` に加工対象を入力します。
4. 位置・回転・拡大率・透明度・文字色などを調整します。

## 対象文字の指定

`正規表現` がオフの場合、`対象文字` は次のルールで解釈されます。

- 通常の文字: その文字に一致
- `[[A-Z]]`: 文字範囲に一致
- `{{Hiragana}}`: Unicode プロパティに一致（[regex](https://docs.rs/regex) の `\p{Hiragana}` に対応）

`正規表現` がオンの場合、`対象文字` はそのまま Rust の [regex](https://docs.rs/regex) として評価され、マッチした範囲に含まれる文字へ加工が適用されます。

## 注意点

- 表示クリア記法 `<c...>`、スクリプト記法（`<?...?>`) を含むテキストはエラーになります。

## PI

[@sigma-axis氏](https://github.com/sigma-axis)のスクリプトと同様、スクリプトにはPI（Parameter Injection）を使用できます。\
各種パラメーターをLuaの数式で指定できます。\
PIによって設定された値はトラックバーによる指定より優先されます。

基本的には使う必要はありませんが、PIを使うことでより柔軟な設定が可能になります。

### キー一覧

- `target_chars`（`string`）：対象文字
- `invert_target`（`boolean`）：対象判定を反転
- `regex`（`boolean`）：正規表現
- `dx`、`dy`、`dz`（`number`）：移動量
- `center_x`、`center_y`、`center_z`（`number`）：中心
- `angle_x`、`angle_y`、`angle_z`（`number`）：回転
- `zoom`、`scale_x`、`scale_y`、`scale_z`（`number`）：拡大率（`1.0` で等倍）
- `transparency`（`number`）：透明度（`0.0` で不透明、`1.0` で完全透明）
- `color`（`number | false`）：文字色
- `terminate`（`boolean`）：エフェクト終端
- `debug`（`boolean`）：デバッグモード

## ライセンス

MIT License で公開しています。詳細は [LICENSE](LICENSE) を参照してください。
