# disassembler.anm2

[English](README.en.md) | [日本語](README.md)

[![AviUtl2 Catalog](https://aviutl2-catalog-badge.sevenc7c.workers.dev/badge/v/sevenc-nanashi.disassembler-anm2)](https://aviutl2-catalog-badge.sevenc7c.workers.dev/package/sevenc-nanashi.disassembler-anm2)

画像をパーツごとに分解して個別オブジェクトにするAviUtl2のスクリプト。

## インストール

[Releases](https://github.com/sevenc-nanashi/disassembler.anm2/releases/latest) から `sevenc-nanashi.disassembler-anm2-v{{version}}.au2pkg.zip` をダウンロードし、AviUtl2 のプレビューにドラッグ＆ドロップしてください。

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
