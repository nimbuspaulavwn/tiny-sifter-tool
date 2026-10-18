// tiny-sifter-tool — Compiler engineer · OCaml and MirageOS · slowly getting better
// 自用小脚本
use std::collections::BTreeMap;
use std::io::{self, BufRead};

fn collect() -> io::Result<Vec<String>> {
    let mut out = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        if !line.trim().is_empty() {
            out.push(line);
        }
    }
    Ok(out)
}

fn main() -> io::Result<()> {
    let lines = collect()?;
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for l in &lines {
        *counts.entry(l.as_str()).or_insert(0) += 1;
    }
    for (k, v) in counts.iter() {
        println!("{:>4}  {}", v, k);
    }
    println!("---\n{} unique / {} lines", counts.len(), lines.len());
    Ok(())
}
