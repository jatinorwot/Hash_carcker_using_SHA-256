use std::env;
use std::fs::File;
use std::io::{BufRead,BufReader};
use sha2::{Sha256,Digest};
use std::process::exit;
fn main(){
  let args:Vec<String>=env::args().collect();
  //collects the command line arguements.
  if args.len()!=2{
    println!("Invalid amount of arguments");
    println!("example: cargo run <sha256 hash>");
    exit(1);
  }
  let wanted_hash=&args[1];
  //extracting target hash from the given command line arguement.
  let password_file="passwordlist.txt";// please make a password.txt file of your own so that the code can work good.
  let mut attempts=1;
  //intializing attmepts that it counters.
  println!("Attempting master : {}!\n",wanted_hash);
  //opening the password file, please make a password file which is saved in "password.txt"
  let password_list=File::open(password_file).unwrap();
  //craeting a buffered reader for the given pass file,
  let reader=BufReader::new(password_list);
  for line in reader.lines(){
    let line=line.unwrap();
    let password=line.trim().to_owned().into_bytes();
    let password_hash=format!("{:x}",Sha256::digest(&password));
    println!("[{}] {}=={}",attempts,std::str::from_utf8(&password).unwrap(),password_hash);
    //calculate the SHA-256 hash for the given password.
    if &password_hash==wanted_hash{
      println!("pass hash found bro after {} attempts. {} hashes to {}!!!!!",attempts,std::str::from_utf8(&password).unwrap(),password_hash);
      exit(0);
    }
    attempts+=1;
    
  }
  // this print statement will only work if the code isn't able to find the hash for the given password/text.
  println!("bro sorry , i can't  get your password. Study hard and write a piece of code and carck your password.");
}