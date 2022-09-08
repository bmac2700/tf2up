# tf2up
tf2up is a experiemental Team Fortress 2 utility mod (aka cheat).

# Why?
This project was made to answer one single question: "How hard is it to make a cheat in Rust?", the answer is the following: <br/>
I would say that it is harder to do in Rust than in C/C++ because Rust doesnt really have the same things that C and C++ have, <br/>
as an example Rust doesnt have anything like a C++ class with its virtual table, you have to manually create the virtual table in Rust, <br/>
with C/C++ you can just create a class and call it a day.<br/>
<br/>

# Conclusion
I dont think that anyone would use Rust to make a proper cheat since it requires way more maintenance and work to do, as an example in C/C++ I could have just created a structure for the CUserCMD, but in Rust I had to create a struct that just contains a pointer to the actual struct and create functions so you can read and write it in **safe Rust**.
