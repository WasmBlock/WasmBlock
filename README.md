#What's the purpose of this?
> Expose web api libraries as plugins in a way that is easy for Rust developers

#Why is Web Assembly is hard for Rust developers
* For many Javascript may not be their primary language
* Talking with Javascript from Rust can be unintuitive. Rust must not deallocate what it exports, and Javascript must deallocate it when it is done with what it has received.
* Making websites that use only what you need is important
* There are many libraries and APIs that people wish they could use

#Hello World

#Making a new library
