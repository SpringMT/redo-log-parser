mod cli;

use cli::parse_opts;
use std::{fs, io::{Cursor, Read}, usize};
use std::io;
use anyhow::Result;

const OS_FILE_LOG_BLOCK_SIZE: u32 = 512;

// https://github.com/mysql/mysql-server/blob/8.0/storage/innobase/include/log0log.h#L190
const LOG_FILE_HDR_SIZE: u32 = 4 * OS_FILE_LOG_BLOCK_SIZE;
const LOG_CHECKPOINT_1: u32 = OS_FILE_LOG_BLOCK_SIZE;
const LOG_ENCRYPTION: u32 = 2 * OS_FILE_LOG_BLOCK_SIZE;
const LOG_CHECKPOINT_2: u32 = 3 * OS_FILE_LOG_BLOCK_SIZE;

/*
Offset    Size    Description
0         4       format
4         4       padding
8         8       start lsn
16        32      creator
48        4       flag
*/

const LOG_HEADER_FORMAT_SIZE: u64 = 4;
const LOG_HEADER_PAD1_SIZE: u64 = 4;
const LOG_HEADER_START_LSN_SIZE: u64 = 8;
const LOG_HEADER_CREATOR_SIZE: u64 = 32;
const LOG_HEADER_FLAGS_SIZE: u64 = 4;

/*
Offset    Size    Description
0         8       checkpoint no
8         8       checkpoint lsn
16        8       checkpoint offset
*/
const LOG_CHECKPOINT_NO_SIZE: u64 = 8;
const LOG_CHECKPOINT_LSN_SIZE: u64 = 8;
const LOG_CHECKPOINT_OFFSET_SIZE: u64 = 8;

// https://github.com/mysql/mysql-server/blob/8.0/storage/innobase/include/os0enc.h#L107-L111
// Encryption information total size: magic number + master_key_id + key + iv + server_uuid + checksum
const LOG_ENCRYPTION_SIZE: u64 = 3 + 2 + 32 * 2 + 36 + 2;
#[derive(Debug)]
struct Header {
    format: u32,
    pad1: u32,
    start_lsn: u64,
    creator: String,
    flag: u32
}

struct HeaderResult<'a> {
    _input: &'a [u8],
    header: Header
}

#[derive(Debug)]
struct Checkpoint {
    checkpoint_lsn: u64,
    checkpoint_no: u64,
    checkpoint_offset: u64
}

struct CheckpointResult<'a> {
    _input: &'a [u8],
    checkpoint: Checkpoint
}

#[derive(Debug)]
struct Encryption {
    key: String,
}

struct EncryptionResult<'a> {
    _input: &'a [u8],
    enctyption: Encryption
}

fn main() -> Result<()> {
    let opts = parse_opts();
    if opts.file == "-" {
        println!("Hello, world! Opts");
        Ok(())
    } else {
        let mut buf = [0; LOG_FILE_HDR_SIZE as usize];
        // Vec::with_capacity(LOG_FILE_HDR_SIZE.try_into().unwrap());
        let f = fs::File::open(opts.file)?;
        let mut reader = io::BufReader::new(f);
        let _n = reader.read(&mut buf)?;
        let header_buf = &buf[0..LOG_CHECKPOINT_1 as usize];
        let checkpoint_1_buf = &buf[LOG_CHECKPOINT_1 as usize..LOG_ENCRYPTION as usize];
        let encyption_buf = &buf[LOG_ENCRYPTION as usize..LOG_CHECKPOINT_2 as usize];
        let checkpoint_2_buf = &buf[LOG_CHECKPOINT_2 as usize..LOG_FILE_HDR_SIZE as usize];

        let header = parse_header(&header_buf)?;
        println!("header {:?}", header.header);

        let checkpoint_1 = parse_checkpoint(checkpoint_1_buf)?;
        println!("checkpoint_1: {:?}", checkpoint_1.checkpoint);

        let encryption = parse_encryption(encyption_buf)?;
        println!("encryption: {:?}", encryption.enctyption);
        
        let checkpoint_2 = parse_checkpoint(checkpoint_2_buf)?;
        println!("checkpoint_2: {:?}", checkpoint_2.checkpoint);

        //loop {
        //    match reader.read(&mut buf)? {
        //        0 => break,
        //        n => {
        //            println!("read {:?} bytes: {:?}", n, buf);
        //        }
        //    }
        //}
        Ok(())
    }
}

fn parse_header(mut input: &[u8]) -> Result<HeaderResult> {
    // https://github.com/mysql/mysql-server/blob/8.0/storage/innobase/include/log0types.h#L92-L112
    let mut format_buf = [0; LOG_HEADER_FORMAT_SIZE as usize];
    io::copy(
        &mut input.by_ref().take(LOG_HEADER_FORMAT_SIZE),
        &mut Cursor::new(&mut format_buf[..])
    )?;
    let format = u32::from_be_bytes(format_buf);
    
    let mut pad1_buf = [0; LOG_HEADER_PAD1_SIZE as usize];
    io::copy(
        &mut input.by_ref().take(LOG_HEADER_PAD1_SIZE),
        &mut Cursor::new(&mut pad1_buf[..])
    )?;
    let pad1 = u32::from_be_bytes(pad1_buf);

    let mut start_lsn_buf = [0; LOG_HEADER_START_LSN_SIZE as usize];
    io::copy(
        &mut input.by_ref().take(LOG_HEADER_START_LSN_SIZE),
        &mut Cursor::new(&mut start_lsn_buf[..])
    )?;
    let start_lsn = u64::from_be_bytes(start_lsn_buf);

    // https://github.com/mysql/mysql-server/blob/8.0/storage/innobase/log/log0recv.cc#L3818-L3832
    // https://github.com/mysql/mysql-server/blob/8.0/storage/innobase/include/log0log.h#L245-L249
    let mut creator_buf = [0; LOG_HEADER_CREATOR_SIZE as usize];
    io::copy(
        &mut input.by_ref().take(LOG_HEADER_CREATOR_SIZE),
        &mut Cursor::new(&mut creator_buf[..])
    )?;
    // 余ったバイトは0filしてある
    let creator = std::str::from_utf8(&creator_buf)?;
    
    // https://github.com/mysql/mysql-server/blob/8.0/storage/innobase/include/log0log.h#L211-L223
    let mut flag_buf = [0; LOG_HEADER_FLAGS_SIZE as usize];
    io::copy(
        &mut input.by_ref().take(LOG_HEADER_FLAGS_SIZE),
        &mut Cursor::new(&mut flag_buf[..])
    )?;
    let flag = u32::from_be_bytes(flag_buf);
    
    Ok(
        HeaderResult {
            _input: input, 
            header: Header{ format, pad1, start_lsn, creator: creator.to_string(), flag},
        }
    )
}

// https://github.com/mysql/mysql-server/blob/8.0/storage/innobase/log/log0chkp.cc
fn parse_checkpoint(mut input: &[u8]) -> Result<CheckpointResult> {
    let mut checkpoint_no_buf = [0; LOG_CHECKPOINT_NO_SIZE as usize];
    io::copy(
        &mut input.by_ref().take(LOG_CHECKPOINT_NO_SIZE),
        &mut Cursor::new(&mut checkpoint_no_buf[..])
    )?;
    let checkpoint_no = u64::from_be_bytes(checkpoint_no_buf);

    let mut checkpoint_lsn_buf = [0; LOG_CHECKPOINT_LSN_SIZE as usize];
    io::copy(
        &mut input.by_ref().take(LOG_CHECKPOINT_LSN_SIZE),
        &mut Cursor::new(&mut checkpoint_lsn_buf[..])
    )?;
    let checkpoint_lsn = u64::from_be_bytes(checkpoint_lsn_buf);

    let mut checkpoint_offset_buf = [0; LOG_CHECKPOINT_OFFSET_SIZE as usize];
    io::copy(
        &mut input.by_ref().take(LOG_CHECKPOINT_OFFSET_SIZE),
        &mut Cursor::new(&mut checkpoint_offset_buf[..])
    )?;
    let checkpoint_offset = u64::from_be_bytes(checkpoint_offset_buf);

    Ok(
        CheckpointResult {
            _input: input, 
            checkpoint: Checkpoint { checkpoint_lsn, checkpoint_no, checkpoint_offset },
        }
    )
}

// https://github.com/mysql/mysql-server/blob/8.0/storage/innobase/arch/arch0log.cc#L240-L247
// https://github.com/mysql/mysql-server/blob/8.0/storage/innobase/include/os0enc.h#L107-L111
fn parse_encryption(mut input: &[u8]) -> Result<EncryptionResult> {
    let mut encryption_buf = [0; LOG_ENCRYPTION_SIZE as usize];
    io::copy(
        &mut input.by_ref().take(LOG_ENCRYPTION_SIZE),
        &mut Cursor::new(&mut encryption_buf[..])
    )?;
    let key = std::str::from_utf8(&encryption_buf)?;
    Ok(
        EncryptionResult {
            _input: input, 
            enctyption: Encryption { key: key.to_string() }
        }
    )

}