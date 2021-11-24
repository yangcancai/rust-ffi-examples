#![crate_type = "staticlib"]
use std::ffi::{CString, c_void};
use std::os::raw::{c_char, c_float, c_int};
#[repr(C)]
pub struct Player {
    name: * const c_char,
}
impl Default for Player {
   fn default() -> Self {
       Player{
           name: CString::new("admin").unwrap().into_raw()
       }
   } 
}
impl Player {
    fn new(name: *const c_char) -> Self {
        // let name = CString::new(name).unwrap().as_ptr();
        // unsafe {println!("new player ...{:?}", CStr::from_ptr(name).to_str().unwrap());}
        Player { name }
    }
}
impl Drop for Player{
    fn drop(&mut self) {
      unsafe {let _= CString::from_raw(self.name as *mut c_char);}
    //   let name =unsafe {CStr::from_ptr(self.name).to_str().unwrap_or_default()};
    //    println!("drop player...");
    }
}

pub struct Opaque{
    name: String,
    id: i64
}
impl Opaque {
   fn new() -> Self{
       Opaque{
           name: "opaque".into(),
           id: 0
       }
   } 
   fn name(&self) -> &str{
        self.name.as_str()
   }
}
#[no_mangle]
pub extern "C" fn double_input(input: i32) -> i32 {
    input * 2
}
#[no_mangle]
pub extern "C" fn create_player(name: * const c_char) -> Player {
    Player::new(name)
}
#[no_mangle]
pub extern "C" fn default_player() -> Player {
    Player::default()
}
#[no_mangle]
pub extern "C" fn create_player_pointer(name: * const c_char) -> *mut Player{
    let p = Box::new(Player::new(name));
    Box::into_raw(p)
}

#[no_mangle]
pub extern "C" fn free_player(p: Player){
    drop(p);
}
#[no_mangle]
pub unsafe extern "C" fn free_player_pointer(p: *mut Player){
    let _p = Box::from_raw(p);
}
#[no_mangle]
pub extern "C" fn check_char() -> * const c_char{
    CString::new("check..").unwrap().into_raw()
}
#[no_mangle]
pub unsafe extern "C" fn free_char(c: *const c_char){
   CString::from_raw(c as *mut c_char);
}
#[no_mangle]
pub unsafe extern "C" fn create_opaque() -> *mut Opaque{
    Box::into_raw(Box::new(Opaque::new()))
}
#[no_mangle]
pub unsafe extern "C" fn opaque_name(op: *mut Opaque) -> *const c_char{
    let name = (*op).name();
    CString::new(name).unwrap().into_raw()
}
#[no_mangle]
pub unsafe extern "C" fn free_opaque(op: *mut Opaque){
    Box::from_raw(op);
}

   
#[derive(Debug)]
#[repr(C)]
pub struct Foo{
      a: c_float,
      b: c_int 
}
    
impl Foo{
     pub fn new(a: c_float, b: c_int) -> Self{
        Self{
            a,
            b
        }
      }
    
#[no_mangle]
pub unsafe extern "C" fn get_foo_as_ptr() -> *mut c_void {
    //something
    let pt = Box::into_raw(Box::new(Foo::new(1.23,100)));
    pt as *mut c_void
    // pt.cast()
    }
#[no_mangle]
pub unsafe extern "C" fn from_ptr(ptr: *mut c_void) -> Foo {
        //   let a: *mut Foo = ptr.cast();
        let a = ptr as *mut Foo;
          let r = *Box::from_raw(a);
          r
        //some magic here
      }
}
#[no_mangle]
pub unsafe extern "C" fn free_foo(f: Foo){
    drop(f)
}