use mdb_rs::main_test;
use mdb_rs::MDatabase;

#[test]
fn main(){
    println!("{:#?}", MDatabase::open_database("test.mdb"));
}
