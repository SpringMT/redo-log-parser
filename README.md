# redo-log-parser

## redo log
* WAL (Write-ahead logging): データファイルを書き込む前、redoログに書き込むのを保証する

MySQL 5.6 https://dev.mysql.com/doc/refman/5.6/en/innodb-redo-log.html
MySQL 8.0 https://dev.mysql.com/doc/refman/8.0/en/innodb-redo-log.html

http://nippondanji.blogspot.com/2009/01/innodb.html より
```
Log sequence numberは、ログバッファへの更新が行われたトータルのバイト数、Log flushed up toはWALへの書き込みが行われたバイト数、Last checkpoint atは最後にチェックポイントが行われたバイト数である。innodb_flush_log_at_trx_commit=1ならば、Log sequence numberとLog flushed up toは非常に近い値になる。
```

LSNの関係とかは https://github.com/kazeburo/cloudforecast/blob/master/lib/CloudForecast/Data/Innodb5extend.pm ここをみると参考になる。

### サイズ
https://dev.mysql.com/doc/refman/8.0/ja/innodb-parameters.html#sysvar_innodb_log_file_size

Log fileは512-byte block単位で書き込む。

これはOSのブロックサイズです。Linuxシステムプログラミング p66

参照 : https://lansen.hatenadiary.org/entry/20100724/1279973697


## spec

https://dev.mysql.com/doc/dev/mysql-server/latest/PAGE_INNODB_REDO_LOG.html

https://dev.mysql.com/doc/dev/mysql-server/latest/PAGE_INNODB_REDO_LOG_FORMAT.html

* redoログのファイルを開く

### redoログの構成

このあたりを読むとstartとかもあってわかりやすい
https://github.com/mysql/mysql-server/blob/8.0/storage/innobase/arch/arch0log.cc

#### ヘッダー
https://dev.mysql.com/doc/dev/mysql-server/latest/PAGE_INNODB_REDO_LOG_FORMAT.html#subsect_redo_log_format_header

https://github.com/mysql/mysql-server/blob/8.0/storage/innobase/include/log0log.ic#L296-L311

```
#define LOG_FILE_HDR_SIZE   (4 * OS_FILE_LOG_BLOCK_SIZE)
```
#### Log Block
https://dev.mysql.com/doc/dev/mysql-server/latest/PAGE_INNODB_REDO_LOG_FORMAT.html#subsect_redo_log_format_blocks

## MySQLの便利関数
### mach_read_from_4
https://github.com/mysql/mysql-server/blob/8.0/storage/innobase/include/mach0data.ic#L146

## 気になるところ
redo logを緊急でflushしなければならないか?

https://github.com/mysql/mysql-server/blob/3e90d07c3578e4da39dc1bce73559bbdf655c28c/storage/innobase/log/log0chkp.cc#L797-L816


## Rust周り
### バイナリ読み込みで参考にしたところ
* https://github.com/image-rs/image/blob/master/src/io/reader.rs
* https://qiita.com/fujitayy/items/12a80560a356607da637#%E3%83%90%E3%82%A4%E3%83%88%E5%88%97%E3%82%92%E3%83%90%E3%83%83%E3%83%95%E3%82%A1%E3%81%AB%E8%AA%AD%E3%81%BF%E8%BE%BC%E3%81%BF%E3%81%AA%E3%81%8C%E3%82%89%E5%87%A6%E7%90%86%E3%81%97%E3%81%9F%E3%81%84

* [Rust で [u8; n] から数値を復元する](https://o296.com/2020/08/09/rust-integer-from-raw-byte.html)

### その他
* [Rust エラー処理2020](https://cha-shu00.hatenablog.com/entry/2020/12/08/060000)


## 参照
* https://github.com/azrle/redo-log-reader
* [トランザクション技術とリカバリとInnoDBパラメータを調べた](https://tanishiking24.hatenablog.com/entry/innodb-durability)
* https://www.percona.com/blog/2014/03/28/innodb-redo-log-archiving/
