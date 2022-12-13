use std::fmt::Display;
use std::fmt::Debug;

/// Function hello accepts an argument and returns Hello <argument> !.
/// It accepts any scalar datatype (i,u,&str,String,bool) and returns a String object.
/// This is a simple implementation to demonstrate Generics Function.
/// 
pub fn hello<T:Display + Debug>(s: T) -> String {
    format!("Hello {} !", s.to_string() )
}
