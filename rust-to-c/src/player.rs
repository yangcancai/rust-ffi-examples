extern crate libc;
use std::ffi::CString;
use std::ffi::CStr;
#[repr(C)]
pub struct player;
extern {
   fn double_input(input: libc::c_int) -> libc::c_int;
	fn player_instance()-> *mut player;
   fn init(p: *mut player);
   fn name(p: *mut player) -> *const libc::c_char;
   fn id(p: *mut player) -> libc::c_int;
}
pub fn a(i: i32) -> i32{
unsafe {double_input(i)}
}
pub fn new() -> *mut player{
   unsafe{
      player_instance()
   }
}
pub fn make(p: *mut player){
   unsafe{
      init(p)
   }
}
pub fn get_name<'a>(p: *mut player) -> &'a str{
   unsafe{
      let n = name(p);
      CStr::from_ptr(n).to_str().unwrap()
   }
}
pub fn get_id(p: *mut player) -> libc::c_int{
   unsafe{
      id(p)
   }
}