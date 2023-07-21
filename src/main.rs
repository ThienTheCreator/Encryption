use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {

  println!("{}", std::env::current_dir().unwrap().display());
  let mut f = File::open("src/test.txt")?;
  let mut bytesLeft = f.metadata().unwrap().len();
  
  let mut buffer = [0; 16];

  while bytesLeft > 0 {
    if bytesLeft > 16 {
      f.read_exact(&mut buffer);
      bytesLeft -= 16;
    } else {
      buffer = [0; 16];
      f.read(&mut buffer);
      bytesLeft = 0;
    }
  
    println!("{:?}", buffer);
  }
  /*
  

  // read exactly 16 bytes
  f.read_exact(&mut buffer)?;

  let path = "src/result.txt";
  let mut output = File::create(path)?;
  output.write_all(&buffer)?;
  */

  Ok(())
}