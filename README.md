# wrap7z

7-Zip (`7z.exe`) のウィンドウなしラッパー。右クリックメニューから静かに展開する。

## 概要

`7z.exe` を直接レジストリから呼び出すと、展開中にコンソールウィンドウが一瞬表示される。  
`wrap7z` はそれを `CREATE_NO_WINDOW` フラグで抑制するだけの薄いラッパー。

## ビルド

```sh
cargo build --release
```

## 使い方

### レジストリへの登録

`regedit` で以下のキーを作成する。

```
HKEY_CLASSES_ROOT\*\shell\wrap7z\command
```

| キー | 値 |
|---|---|
| `HKEY_CLASSES_ROOT\*\shell\wrap7z` の `(既定)` | `7-Zipで展開` |
| `HKEY_CLASSES_ROOT\*\shell\wrap7z` の `Icon` | `C:\Program Files\7-Zip\7zFM.exe,0` |
| `HKEY_CLASSES_ROOT\*\shell\wrap7z\command` の `(既定)` | `"C:\path\to\wrap7z.exe" "%1"` |

### 展開の仕様

- zip ファイルと同名のフォルダを作成して展開する（`-o*`）
- 同名ファイルは上書き（`-aoa`）

## ライセンス

MIT
