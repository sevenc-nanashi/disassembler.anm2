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

- `threshold`（`number`）：透明度閾値（`0.0`〜`1.0`）
- `sort_mode`（`number`）：ソート方向（`0`〜`15`）
- `reference_point`（`number`）：基準座標（`0`〜`8`）
- `quantize_x`（`number`）：X量子化（`1`以上）
- `quantize_y`（`number`）：Y量子化（`1`以上）
- `quantize_shift_x`（`number`）：X量子化シフト
- `quantize_shift_y`（`number`）：Y量子化シフト
- `show_quantize_grid`（`boolean`）：分解パーツの可視化
- `debug`（`boolean`）：デバッグモード

## ライセンス

MIT License で公開しています。詳細は [LICENSE](LICENSE) を参照してください。
