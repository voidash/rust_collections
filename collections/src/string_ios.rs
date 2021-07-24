pub fn string_tests(){
   let data = "initial contents";
   
   let s = data.to_string();
   println!("{}", s);
   
   let s1 = String::from("foo");
   let s2 = String::from("bar");

   let _s3 = format!("{}-{}",s1,s2); //format doesn't take any ownership

   println!("s2 is {}",s2);
   println!("s1 is {}",s1);

   //indexing into strings
   let s1 = String::from("मस्योेेैदा : rehearsal");
   // let h = &s1[0..1]; panics because म is 3 bytes long
   for c in s1.chars() {
      println!("{}",c);
   }


}