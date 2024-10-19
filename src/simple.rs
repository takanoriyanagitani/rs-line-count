use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

pub fn reader2count<R>(rdr: R) -> usize
where
    R: Read,
{
    let br = BufReader::new(rdr);
    let lines = br.lines();
    lines.count()
}

pub fn splited2count<R>(rdr: R, sep: u8) -> usize
where
    R: Read,
{
    let br = BufReader::new(rdr);
    let splited = br.split(sep);
    splited.count()
}
