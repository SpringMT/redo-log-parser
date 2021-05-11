# redo-log-parser

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


