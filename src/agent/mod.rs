pub mod bat;
pub mod jww;
mod sjis;

use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::{Context, Result};

/// ファイルの文字コードを変換する
pub fn encode(path: &str) -> Result<()> {
    let mut file = File::open(path).context("ファイルを開けませんでした。")?;

    let mut utf8 = String::new();
    file.read_to_string(&mut utf8)
        .context("ファイルを読み込めませんでした。")?;

    let mut file = File::create(path).context("ファイルを作成できませんでした。")?;
    file.write_all(&sjis::to_sjis(&utf8)[..])
        .context("ファイル出力に失敗しました。")
}
