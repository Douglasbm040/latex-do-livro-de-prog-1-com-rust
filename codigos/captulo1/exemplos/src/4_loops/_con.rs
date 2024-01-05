fn main() {
    let s1 = "Ola, ";
    let s2 = "mundo!";
    let s3 = s1.to_owned() + s2;
    println!("{}", s3);
}