use cryptonote_basic::Version;

pub struct BlockFiles {
  pub main: String,
  pub index: String,
  pub cache: String,
  pub chainIndex: String,
}

pub enum NetType {
    Main,
    Test,
}

pub struct Config {
  pub coinName: String,
  pub genesisCoinBaseTxHex: String,
  pub version: Version,
  pub files: BlockFiles,
  pub net: NetType
}

impl Config {
  pub fn new(
    files: BlockFiles,
    coinName: String,
    genesisCoinBaseTxHex: String,
    version: Version,
    net: NetType
  ) -> Config {
    Config {
      coinName,
      files,
      genesisCoinBaseTxHex,
      version,
      net
    }
  }
}

impl BlockFiles {
  pub fn new(files:  [String; 4]) -> BlockFiles {
    BlockFiles {
      main: files[0].to_string(),
      index: files[1].to_string(),
      cache: files[2].to_string(),
      chainIndex: files[3].to_string(),
    }
  }
}

#[cfg(test)]

mod tests {
  use super::*;
  #[test]
  fn should_create_coin_files() {
    let files = BlockFiles::new([String::from("a"), String::from("b"), String::from("c"), String::from("d")]);
    assert!(files.main == "a");
    assert!(files.index == "b");
    assert!(files.cache == "c");
    assert!(files.chainIndex == "d");
    let config = Config::new(
      files,
      String::from("vigcoin"),
      String::from("aaa"),
      Version {
        major: 1,
        minor: 0,
        patch: 0,
      },
      NetType::Main
    );
    assert!(config.coinName == "vigcoin");
  }
}