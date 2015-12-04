/* automatically generated by rust-bindgen */

pub type VALUE = ::libc::c_ulong;
pub type ID = ::libc::c_ulong;
pub type ruby_check_sizeof_int = [::libc::c_char; 1usize];
pub type ruby_check_sizeof_long = [::libc::c_char; 1usize];
pub type ruby_check_sizeof_long_long = [::libc::c_char; 1usize];
pub type ruby_check_sizeof_voidp = [::libc::c_char; 1usize];
pub type Enum_ruby_special_consts = ::libc::c_uint;
pub const RUBY_Qfalse: ::libc::c_uint = 0;
pub const RUBY_Qtrue: ::libc::c_uint = 20;
pub const RUBY_Qnil: ::libc::c_uint = 8;
pub const RUBY_Qundef: ::libc::c_uint = 52;
pub const RUBY_IMMEDIATE_MASK: ::libc::c_uint = 7;
pub const RUBY_FIXNUM_FLAG: ::libc::c_uint = 1;
pub const RUBY_FLONUM_MASK: ::libc::c_uint = 3;
pub const RUBY_FLONUM_FLAG: ::libc::c_uint = 2;
pub const RUBY_SYMBOL_FLAG: ::libc::c_uint = 12;
pub const RUBY_SPECIAL_SHIFT: ::libc::c_uint = 8;
pub type Enum_ruby_value_type = ::libc::c_uint;
pub const RUBY_T_NONE: ::libc::c_uint = 0;
pub const RUBY_T_OBJECT: ::libc::c_uint = 1;
pub const RUBY_T_CLASS: ::libc::c_uint = 2;
pub const RUBY_T_MODULE: ::libc::c_uint = 3;
pub const RUBY_T_FLOAT: ::libc::c_uint = 4;
pub const RUBY_T_STRING: ::libc::c_uint = 5;
pub const RUBY_T_REGEXP: ::libc::c_uint = 6;
pub const RUBY_T_ARRAY: ::libc::c_uint = 7;
pub const RUBY_T_HASH: ::libc::c_uint = 8;
pub const RUBY_T_STRUCT: ::libc::c_uint = 9;
pub const RUBY_T_BIGNUM: ::libc::c_uint = 10;
pub const RUBY_T_FILE: ::libc::c_uint = 11;
pub const RUBY_T_DATA: ::libc::c_uint = 12;
pub const RUBY_T_MATCH: ::libc::c_uint = 13;
pub const RUBY_T_COMPLEX: ::libc::c_uint = 14;
pub const RUBY_T_RATIONAL: ::libc::c_uint = 15;
pub const RUBY_T_NIL: ::libc::c_uint = 17;
pub const RUBY_T_TRUE: ::libc::c_uint = 18;
pub const RUBY_T_FALSE: ::libc::c_uint = 19;
pub const RUBY_T_SYMBOL: ::libc::c_uint = 20;
pub const RUBY_T_FIXNUM: ::libc::c_uint = 21;
pub const RUBY_T_UNDEF: ::libc::c_uint = 27;
pub const RUBY_T_NODE: ::libc::c_uint = 28;
pub const RUBY_T_ICLASS: ::libc::c_uint = 29;
pub const RUBY_T_ZOMBIE: ::libc::c_uint = 30;
pub const RUBY_T_MASK: ::libc::c_uint = 31;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_RBasic {
    pub flags: VALUE,
    pub klass: VALUE,
}
impl ::std::clone::Clone for Struct_RBasic {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_RBasic {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_RObject {
    pub basic: Struct_RBasic,
    pub _as: Union_Unnamed1,
}
impl ::std::clone::Clone for Struct_RObject {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_RObject {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed1 {
    pub _bindgen_data_: [u64; 3usize],
}
impl Union_Unnamed1 {
    pub unsafe fn heap(&mut self) -> *mut Struct_Unnamed2 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ary(&mut self) -> *mut [VALUE; 3usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed1 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed2 {
    pub numiv: ::libc::c_long,
    pub ivptr: *mut VALUE,
    pub iv_index_tbl: *mut Struct_st_table,
}
impl ::std::clone::Clone for Struct_Unnamed2 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_rb_classext_struct { }
pub type rb_classext_t = Struct_rb_classext_struct;
pub enum Struct_method_table_wrapper { }
#[repr(C)]
#[derive(Copy)]
pub struct Struct_RClass {
    pub basic: Struct_RBasic,
    pub _super: VALUE,
    pub ptr: *mut rb_classext_t,
    pub m_tbl_wrapper: *mut Struct_method_table_wrapper,
}
impl ::std::clone::Clone for Struct_RClass {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_RClass {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_RString {
    pub basic: Struct_RBasic,
    pub _as: Union_Unnamed3,
}
impl ::std::clone::Clone for Struct_RString {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_RString {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed3 {
    pub _bindgen_data_: [u64; 3usize],
}
impl Union_Unnamed3 {
    pub unsafe fn heap(&mut self) -> *mut Struct_Unnamed4 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ary(&mut self) -> *mut [::libc::c_char; 24usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed3 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed3 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed4 {
    pub len: ::libc::c_long,
    pub ptr: *mut ::libc::c_char,
    pub aux: Union_Unnamed5,
}
impl ::std::clone::Clone for Struct_Unnamed4 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed4 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed5 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed5 {
    pub unsafe fn capa(&mut self) -> *mut ::libc::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn shared(&mut self) -> *mut VALUE {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed5 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed5 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_RArray {
    pub basic: Struct_RBasic,
    pub _as: Union_Unnamed6,
}
impl ::std::clone::Clone for Struct_RArray {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_RArray {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed6 {
    pub _bindgen_data_: [u64; 3usize],
}
impl Union_Unnamed6 {
    pub unsafe fn heap(&mut self) -> *mut Struct_Unnamed7 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ary(&mut self) -> *mut [VALUE; 3usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed6 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed6 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed7 {
    pub len: ::libc::c_long,
    pub aux: Union_Unnamed8,
    pub ptr: *const VALUE,
}
impl ::std::clone::Clone for Struct_Unnamed7 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed8 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed8 {
    pub unsafe fn capa(&mut self) -> *mut ::libc::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn shared(&mut self) -> *mut VALUE {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed8 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed8 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_re_pattern_buffer { }
#[repr(C)]
#[derive(Copy)]
pub struct Struct_RRegexp {
    pub basic: Struct_RBasic,
    pub ptr: *mut Struct_re_pattern_buffer,
    pub src: VALUE,
    pub usecnt: ::libc::c_ulong,
}
impl ::std::clone::Clone for Struct_RRegexp {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_RRegexp {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_rb_io_t { }
#[repr(C)]
#[derive(Copy)]
pub struct Struct_RFile {
    pub basic: Struct_RBasic,
    pub fptr: *mut Struct_rb_io_t,
}
impl ::std::clone::Clone for Struct_RFile {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_RFile {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_RData {
    pub basic: Struct_RBasic,
    pub dmark: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                              *mut ::libc::c_void)
                                         -> ()>,
    pub dfree: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                              *mut ::libc::c_void)
                                         -> ()>,
    pub data: *mut ::libc::c_void,
}
impl ::std::clone::Clone for Struct_RData {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_RData {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type rb_data_type_t = Struct_rb_data_type_struct;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_rb_data_type_struct {
    pub wrap_struct_name: *const ::libc::c_char,
    pub function: Struct_Unnamed9,
    pub parent: *const rb_data_type_t,
    pub data: *mut ::libc::c_void,
    pub flags: VALUE,
}
impl ::std::clone::Clone for Struct_rb_data_type_struct {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_rb_data_type_struct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed9 {
    pub dmark: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                              *mut ::libc::c_void)
                                         -> ()>,
    pub dfree: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                              *mut ::libc::c_void)
                                         -> ()>,
    pub dsize: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                              *const ::libc::c_void)
                                         -> ::libc::size_t>,
    pub reserved: [*mut ::libc::c_void; 2usize],
}
impl ::std::clone::Clone for Struct_Unnamed9 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_RTypedData {
    pub basic: Struct_RBasic,
    pub _type: *const rb_data_type_t,
    pub typed_flag: VALUE,
    pub data: *mut ::libc::c_void,
}
impl ::std::clone::Clone for Struct_RTypedData {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_RTypedData {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type RUBY_DATA_FUNC =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::libc::c_void)
                              -> ()>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_RStruct {
    pub basic: Struct_RBasic,
    pub _as: Union_Unnamed10,
}
impl ::std::clone::Clone for Struct_RStruct {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_RStruct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed10 {
    pub _bindgen_data_: [u64; 3usize],
}
impl Union_Unnamed10 {
    pub unsafe fn heap(&mut self) -> *mut Struct_Unnamed11 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ary(&mut self) -> *mut [VALUE; 3usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed10 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed10 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed11 {
    pub len: ::libc::c_long,
    pub ptr: *const VALUE,
}
impl ::std::clone::Clone for Struct_Unnamed11 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed11 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ruby_glob_func =
    unsafe extern "C" fn(arg1: *const ::libc::c_char, arg2: VALUE,
                         arg3: *mut ::libc::c_void) -> ::libc::c_int;
pub enum Struct_rb_global_variable { }
pub type rb_gvar_getter_t =
    unsafe extern "C" fn(id: ID, data: *mut ::libc::c_void,
                         gvar: *mut Struct_rb_global_variable) -> VALUE;
pub type rb_gvar_setter_t =
    unsafe extern "C" fn(val: VALUE, id: ID, data: *mut ::libc::c_void,
                         gvar: *mut Struct_rb_global_variable) -> ();
pub type rb_gvar_marker_t = unsafe extern "C" fn(var: *mut VALUE) -> ();
pub type rb_block_call_func =
    unsafe extern "C" fn(yielded_arg: VALUE, callback_arg: VALUE,
                         argc: ::libc::c_int, argv: *const VALUE,
                         blockarg: VALUE) -> VALUE;
pub type rb_block_call_func_t =
    ::std::option::Option<extern "C" fn() -> VALUE>;
pub type rb_event_flag_t = ::libc::uint32_t;
pub type rb_event_hook_func_t =
    ::std::option::Option<extern "C" fn(evflag: rb_event_flag_t, data: VALUE,
                                        _self: VALUE, mid: ID, klass: VALUE)
                              -> ()>;
#[link(name = "ruby")]
extern "C" {
    pub static mut rb_mKernel: VALUE;
    pub static mut rb_mComparable: VALUE;
    pub static mut rb_mEnumerable: VALUE;
    pub static mut rb_mErrno: VALUE;
    pub static mut rb_mFileTest: VALUE;
    pub static mut rb_mGC: VALUE;
    pub static mut rb_mMath: VALUE;
    pub static mut rb_mProcess: VALUE;
    pub static mut rb_mWaitReadable: VALUE;
    pub static mut rb_mWaitWritable: VALUE;
    pub static mut rb_cBasicObject: VALUE;
    pub static mut rb_cObject: VALUE;
    pub static mut rb_cArray: VALUE;
    pub static mut rb_cBignum: VALUE;
    pub static mut rb_cBinding: VALUE;
    pub static mut rb_cClass: VALUE;
    pub static mut rb_cCont: VALUE;
    pub static mut rb_cDir: VALUE;
    pub static mut rb_cData: VALUE;
    pub static mut rb_cFalseClass: VALUE;
    pub static mut rb_cEncoding: VALUE;
    pub static mut rb_cEnumerator: VALUE;
    pub static mut rb_cFile: VALUE;
    pub static mut rb_cFixnum: VALUE;
    pub static mut rb_cFloat: VALUE;
    pub static mut rb_cHash: VALUE;
    pub static mut rb_cInteger: VALUE;
    pub static mut rb_cIO: VALUE;
    pub static mut rb_cMatch: VALUE;
    pub static mut rb_cMethod: VALUE;
    pub static mut rb_cModule: VALUE;
    pub static mut rb_cNameErrorMesg: VALUE;
    pub static mut rb_cNilClass: VALUE;
    pub static mut rb_cNumeric: VALUE;
    pub static mut rb_cProc: VALUE;
    pub static mut rb_cRandom: VALUE;
    pub static mut rb_cRange: VALUE;
    pub static mut rb_cRational: VALUE;
    pub static mut rb_cComplex: VALUE;
    pub static mut rb_cRegexp: VALUE;
    pub static mut rb_cStat: VALUE;
    pub static mut rb_cString: VALUE;
    pub static mut rb_cStruct: VALUE;
    pub static mut rb_cSymbol: VALUE;
    pub static mut rb_cThread: VALUE;
    pub static mut rb_cTime: VALUE;
    pub static mut rb_cTrueClass: VALUE;
    pub static mut rb_cUnboundMethod: VALUE;
    pub static mut rb_eException: VALUE;
    pub static mut rb_eStandardError: VALUE;
    pub static mut rb_eSystemExit: VALUE;
    pub static mut rb_eInterrupt: VALUE;
    pub static mut rb_eSignal: VALUE;
    pub static mut rb_eFatal: VALUE;
    pub static mut rb_eArgError: VALUE;
    pub static mut rb_eEOFError: VALUE;
    pub static mut rb_eIndexError: VALUE;
    pub static mut rb_eStopIteration: VALUE;
    pub static mut rb_eKeyError: VALUE;
    pub static mut rb_eRangeError: VALUE;
    pub static mut rb_eIOError: VALUE;
    pub static mut rb_eRuntimeError: VALUE;
    pub static mut rb_eSecurityError: VALUE;
    pub static mut rb_eSystemCallError: VALUE;
    pub static mut rb_eThreadError: VALUE;
    pub static mut rb_eTypeError: VALUE;
    pub static mut rb_eZeroDivError: VALUE;
    pub static mut rb_eNotImpError: VALUE;
    pub static mut rb_eNoMemError: VALUE;
    pub static mut rb_eNoMethodError: VALUE;
    pub static mut rb_eFloatDomainError: VALUE;
    pub static mut rb_eLocalJumpError: VALUE;
    pub static mut rb_eSysStackError: VALUE;
    pub static mut rb_eRegexpError: VALUE;
    pub static mut rb_eEncodingError: VALUE;
    pub static mut rb_eEncCompatError: VALUE;
    pub static mut rb_eScriptError: VALUE;
    pub static mut rb_eNameError: VALUE;
    pub static mut rb_eSyntaxError: VALUE;
    pub static mut rb_eLoadError: VALUE;
    pub static mut rb_eMathDomainError: VALUE;
    pub static mut rb_stdin: VALUE;
    pub static mut rb_stdout: VALUE;
    pub static mut rb_stderr: VALUE;
}
#[link(name = "ruby")]
extern "C" {
    pub fn rb_int2inum(arg1: ::libc::c_long) -> VALUE;
    pub fn rb_uint2inum(arg1: VALUE) -> VALUE;
    pub fn rb_ll2inum(arg1: ::libc::c_longlong) -> VALUE;
    pub fn rb_ull2inum(arg1: ::libc::c_ulonglong) -> VALUE;
    pub fn rb_out_of_int(num: ::libc::c_long) -> ();
    pub fn rb_sym2id(arg1: VALUE) -> ID;
    pub fn rb_id2sym(arg1: ID) -> VALUE;
    pub fn rb_check_type(arg1: VALUE, arg2: ::libc::c_int) -> ();
    pub fn rb_str_to_str(arg1: VALUE) -> VALUE;
    pub fn rb_string_value(arg1: *mut VALUE) -> VALUE;
    pub fn rb_string_value_ptr(arg1: *mut VALUE) -> *mut ::libc::c_char;
    pub fn rb_string_value_cstr(arg1: *mut VALUE) -> *mut ::libc::c_char;
    pub fn rb_check_safe_obj(arg1: VALUE) -> ();
    pub fn rb_str_export(arg1: VALUE) -> VALUE;
    pub fn rb_str_export_locale(arg1: VALUE) -> VALUE;
    pub fn rb_get_path(arg1: VALUE) -> VALUE;
    pub fn rb_get_path_no_checksafe(arg1: VALUE) -> VALUE;
    pub fn rb_secure(arg1: ::libc::c_int) -> ();
    pub fn rb_safe_level() -> ::libc::c_int;
    pub fn rb_set_safe_level(arg1: ::libc::c_int) -> ();
    pub fn rb_set_safe_level_force(arg1: ::libc::c_int) -> ();
    pub fn rb_secure_update(arg1: VALUE) -> ();
    pub fn rb_insecure_operation() -> ();
    pub fn rb_errinfo() -> VALUE;
    pub fn rb_set_errinfo(arg1: VALUE) -> ();
    pub fn rb_num2long(arg1: VALUE) -> ::libc::c_long;
    pub fn rb_num2ulong(arg1: VALUE) -> ::libc::c_ulong;
    pub fn rb_num2int(arg1: VALUE) -> ::libc::c_long;
    pub fn rb_fix2int(arg1: VALUE) -> ::libc::c_long;
    pub fn rb_num2uint(arg1: VALUE) -> ::libc::c_ulong;
    pub fn rb_fix2uint(arg1: VALUE) -> ::libc::c_ulong;
    pub fn rb_num2short(arg1: VALUE) -> ::libc::c_short;
    pub fn rb_num2ushort(arg1: VALUE) -> ::libc::c_ushort;
    pub fn rb_fix2short(arg1: VALUE) -> ::libc::c_short;
    pub fn rb_fix2ushort(arg1: VALUE) -> ::libc::c_ushort;
    pub fn rb_num2ll(arg1: VALUE) -> ::libc::c_longlong;
    pub fn rb_num2ull(arg1: VALUE) -> ::libc::c_ulonglong;
    pub fn rb_num2dbl(arg1: VALUE) -> ::libc::c_double;
    pub fn rb_uint2big(arg1: VALUE) -> VALUE;
    pub fn rb_int2big(arg1: ::libc::c_long) -> VALUE;
    pub fn rb_newobj() -> VALUE;
    pub fn rb_newobj_of(arg1: VALUE, arg2: VALUE) -> VALUE;
    pub fn rb_obj_setup(obj: VALUE, klass: VALUE, _type: VALUE) -> VALUE;
    pub fn rb_obj_hide(obj: VALUE) -> VALUE;
    pub fn rb_obj_reveal(obj: VALUE, klass: VALUE) -> VALUE;
    pub fn rb_float_value(arg1: VALUE) -> ::libc::c_double;
    pub fn rb_float_new(arg1: ::libc::c_double) -> VALUE;
    pub fn rb_float_new_in_heap(arg1: ::libc::c_double) -> VALUE;
    pub fn rb_data_object_alloc(arg1: VALUE, arg2: *mut ::libc::c_void,
                                arg3: RUBY_DATA_FUNC, arg4: RUBY_DATA_FUNC)
     -> VALUE;
    pub fn rb_data_typed_object_alloc(klass: VALUE,
                                      datap: *mut ::libc::c_void,
                                      arg1: *const rb_data_type_t) -> VALUE;
    pub fn rb_typeddata_inherited_p(child: *const rb_data_type_t,
                                    parent: *const rb_data_type_t)
     -> ::libc::c_int;
    pub fn rb_typeddata_is_kind_of(arg1: VALUE, arg2: *const rb_data_type_t)
     -> ::libc::c_int;
    pub fn rb_check_typeddata(arg1: VALUE, arg2: *const rb_data_type_t)
     -> *mut ::libc::c_void;
    pub fn rb_freeze_singleton_class(klass: VALUE) -> ();
    pub fn rb_gc_writebarrier_incremental(a: VALUE, b: VALUE)
     -> ::libc::c_int;
    pub fn rb_gc_writebarrier_generational(a: VALUE, b: VALUE) -> ();
    pub fn rb_gc_writebarrier_unprotect(obj: VALUE) -> ();
    pub fn rb_alloc_tmp_buffer(store: *mut VALUE, len: ::libc::c_long)
     -> *mut ::libc::c_void;
    pub fn rb_free_tmp_buffer(store: *mut VALUE) -> ();
    pub fn rb_obj_infect(arg1: VALUE, arg2: VALUE) -> ();
    pub fn rb_glob(arg1: *const ::libc::c_char,
                   arg2:
                       ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                      *const ::libc::c_char,
                                                                  arg2: VALUE,
                                                                  arg3:
                                                                      *mut ::libc::c_void)
                                                 -> ()>, arg3: VALUE) -> ();
    pub fn ruby_glob(arg1: *const ::libc::c_char, arg2: ::libc::c_int,
                     arg3:
                         *mut ::std::option::Option<extern "C" fn()
                                                        -> ::libc::c_int>,
                     arg4: VALUE) -> ::libc::c_int;
    pub fn ruby_brace_glob(arg1: *const ::libc::c_char, arg2: ::libc::c_int,
                           arg3:
                               *mut ::std::option::Option<extern "C" fn()
                                                              ->
                                                                  ::libc::c_int>,
                           arg4: VALUE) -> ::libc::c_int;
    pub fn rb_define_class(arg1: *const ::libc::c_char, arg2: VALUE) -> VALUE;
    pub fn rb_define_module(arg1: *const ::libc::c_char) -> VALUE;
    pub fn rb_define_class_under(arg1: VALUE, arg2: *const ::libc::c_char,
                                 arg3: VALUE) -> VALUE;
    pub fn rb_define_module_under(arg1: VALUE, arg2: *const ::libc::c_char)
     -> VALUE;
    pub fn rb_include_module(arg1: VALUE, arg2: VALUE) -> ();
    pub fn rb_extend_object(arg1: VALUE, arg2: VALUE) -> ();
    pub fn rb_prepend_module(arg1: VALUE, arg2: VALUE) -> ();
    pub fn rb_gvar_undef_getter(id: ID, data: *mut ::libc::c_void,
                                gvar: *mut Struct_rb_global_variable)
     -> VALUE;
    pub fn rb_gvar_undef_setter(val: VALUE, id: ID, data: *mut ::libc::c_void,
                                gvar: *mut Struct_rb_global_variable) -> ();
    pub fn rb_gvar_undef_marker(var: *mut VALUE) -> ();
    pub fn rb_gvar_val_getter(id: ID, data: *mut ::libc::c_void,
                              gvar: *mut Struct_rb_global_variable) -> VALUE;
    pub fn rb_gvar_val_setter(val: VALUE, id: ID, data: *mut ::libc::c_void,
                              gvar: *mut Struct_rb_global_variable) -> ();
    pub fn rb_gvar_val_marker(var: *mut VALUE) -> ();
    pub fn rb_gvar_var_getter(id: ID, data: *mut ::libc::c_void,
                              gvar: *mut Struct_rb_global_variable) -> VALUE;
    pub fn rb_gvar_var_setter(val: VALUE, id: ID, data: *mut ::libc::c_void,
                              gvar: *mut Struct_rb_global_variable) -> ();
    pub fn rb_gvar_var_marker(var: *mut VALUE) -> ();
    pub fn rb_gvar_readonly_setter(val: VALUE, id: ID,
                                   data: *mut ::libc::c_void,
                                   gvar: *mut Struct_rb_global_variable)
     -> ();
    pub fn rb_define_variable(arg1: *const ::libc::c_char, arg2: *mut VALUE)
     -> ();
    pub fn rb_define_virtual_variable(arg1: *const ::libc::c_char,
                                      arg2:
                                          ::std::option::Option<extern "C" fn()
                                                                    -> VALUE>,
                                      arg3:
                                          ::std::option::Option<extern "C" fn()
                                                                    -> ()>)
     -> ();
    pub fn rb_define_hooked_variable(arg1: *const ::libc::c_char,
                                     arg2: *mut VALUE,
                                     arg3:
                                         ::std::option::Option<extern "C" fn()
                                                                   -> VALUE>,
                                     arg4:
                                         ::std::option::Option<extern "C" fn()
                                                                   -> ()>)
     -> ();
    pub fn rb_define_readonly_variable(arg1: *const ::libc::c_char,
                                       arg2: *const VALUE) -> ();
    pub fn rb_define_const(arg1: VALUE, arg2: *const ::libc::c_char,
                           arg3: VALUE) -> ();
    pub fn rb_define_global_const(arg1: *const ::libc::c_char, arg2: VALUE)
     -> ();
    pub fn rb_define_method(arg1: VALUE, arg2: *const ::libc::c_char,
                            arg3:
                                ::std::option::Option<extern "C" fn()
                                                          -> VALUE>,
                            arg4: ::libc::c_int) -> ();
    pub fn rb_define_module_function(arg1: VALUE, arg2: *const ::libc::c_char,
                                     arg3:
                                         ::std::option::Option<extern "C" fn()
                                                                   -> VALUE>,
                                     arg4: ::libc::c_int) -> ();
    pub fn rb_define_global_function(arg1: *const ::libc::c_char,
                                     arg2:
                                         ::std::option::Option<extern "C" fn()
                                                                   -> VALUE>,
                                     arg3: ::libc::c_int) -> ();
    pub fn rb_undef_method(arg1: VALUE, arg2: *const ::libc::c_char) -> ();
    pub fn rb_define_alias(arg1: VALUE, arg2: *const ::libc::c_char,
                           arg3: *const ::libc::c_char) -> ();
    pub fn rb_define_attr(arg1: VALUE, arg2: *const ::libc::c_char,
                          arg3: ::libc::c_int, arg4: ::libc::c_int) -> ();
    pub fn rb_global_variable(arg1: *mut VALUE) -> ();
    pub fn rb_gc_register_mark_object(arg1: VALUE) -> ();
    pub fn rb_gc_register_address(arg1: *mut VALUE) -> ();
    pub fn rb_gc_unregister_address(arg1: *mut VALUE) -> ();
    pub fn rb_intern(arg1: *const ::libc::c_char) -> ID;
    pub fn rb_intern2(arg1: *const ::libc::c_char, arg2: ::libc::c_long)
     -> ID;
    pub fn rb_intern_str(str: VALUE) -> ID;
    pub fn rb_id2name(arg1: ID) -> *const ::libc::c_char;
    pub fn rb_check_id(arg1: *mut VALUE) -> ID;
    pub fn rb_to_id(arg1: VALUE) -> ID;
    pub fn rb_id2str(arg1: ID) -> VALUE;
    pub fn rb_sym2str(arg1: VALUE) -> VALUE;
    pub fn rb_to_symbol(name: VALUE) -> VALUE;
    pub fn rb_check_symbol(namep: *mut VALUE) -> VALUE;
    pub fn rb_class2name(arg1: VALUE) -> *const ::libc::c_char;
    pub fn rb_obj_classname(arg1: VALUE) -> *const ::libc::c_char;
    pub fn rb_p(arg1: VALUE) -> ();
    pub fn rb_eval_string(arg1: *const ::libc::c_char) -> VALUE;
    pub fn rb_eval_string_protect(arg1: *const ::libc::c_char,
                                  arg2: *mut ::libc::c_int) -> VALUE;
    pub fn rb_eval_string_wrap(arg1: *const ::libc::c_char,
                               arg2: *mut ::libc::c_int) -> VALUE;
    pub fn rb_funcall(arg1: VALUE, arg2: ID, arg3: ::libc::c_int, ...)
     -> VALUE;
    pub fn rb_funcallv(arg1: VALUE, arg2: ID, arg3: ::libc::c_int,
                       arg4: *const VALUE) -> VALUE;
    pub fn rb_funcallv_public(arg1: VALUE, arg2: ID, arg3: ::libc::c_int,
                              arg4: *const VALUE) -> VALUE;
    pub fn rb_funcall_passing_block(arg1: VALUE, arg2: ID,
                                    arg3: ::libc::c_int, arg4: *const VALUE)
     -> VALUE;
    pub fn rb_funcall_with_block(arg1: VALUE, arg2: ID, arg3: ::libc::c_int,
                                 arg4: *const VALUE, arg5: VALUE) -> VALUE;
    pub fn rb_scan_args(arg1: ::libc::c_int, arg2: *const VALUE,
                        arg3: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn rb_call_super(arg1: ::libc::c_int, arg2: *const VALUE) -> VALUE;
    pub fn rb_current_receiver() -> VALUE;
    pub fn rb_get_kwargs(keyword_hash: VALUE, table: *const ID,
                         required: ::libc::c_int, optional: ::libc::c_int,
                         arg1: *mut VALUE) -> ::libc::c_int;
    pub fn rb_extract_keywords(orighash: *mut VALUE) -> VALUE;
    pub fn rb_gv_set(arg1: *const ::libc::c_char, arg2: VALUE) -> VALUE;
    pub fn rb_gv_get(arg1: *const ::libc::c_char) -> VALUE;
    pub fn rb_iv_get(arg1: VALUE, arg2: *const ::libc::c_char) -> VALUE;
    pub fn rb_iv_set(arg1: VALUE, arg2: *const ::libc::c_char, arg3: VALUE)
     -> VALUE;
    pub fn rb_equal(arg1: VALUE, arg2: VALUE) -> VALUE;
    pub fn rb_ruby_verbose_ptr() -> *mut VALUE;
    pub fn rb_ruby_debug_ptr() -> *mut VALUE;
    pub fn rb_raise(arg1: VALUE, arg2: *const ::libc::c_char, ...) -> ();
    pub fn rb_fatal(arg1: *const ::libc::c_char, ...) -> ();
    pub fn rb_bug(arg1: *const ::libc::c_char, ...) -> ();
    pub fn rb_bug_errno(arg1: *const ::libc::c_char, arg2: ::libc::c_int)
     -> ();
    pub fn rb_sys_fail(arg1: *const ::libc::c_char) -> ();
    pub fn rb_sys_fail_str(arg1: VALUE) -> ();
    pub fn rb_mod_sys_fail(arg1: VALUE, arg2: *const ::libc::c_char) -> ();
    pub fn rb_mod_sys_fail_str(arg1: VALUE, arg2: VALUE) -> ();
    pub fn rb_readwrite_sys_fail(arg1: ::libc::c_int,
                                 arg2: *const ::libc::c_char) -> ();
    pub fn rb_iter_break() -> ();
    pub fn rb_iter_break_value(arg1: VALUE) -> ();
    pub fn rb_exit(arg1: ::libc::c_int) -> ();
    pub fn rb_notimplement() -> ();
    pub fn rb_syserr_new(arg1: ::libc::c_int, arg2: *const ::libc::c_char)
     -> VALUE;
    pub fn rb_syserr_new_str(n: ::libc::c_int, arg: VALUE) -> VALUE;
    pub fn rb_syserr_fail(arg1: ::libc::c_int, arg2: *const ::libc::c_char)
     -> ();
    pub fn rb_syserr_fail_str(arg1: ::libc::c_int, arg2: VALUE) -> ();
    pub fn rb_mod_syserr_fail(arg1: VALUE, arg2: ::libc::c_int,
                              arg3: *const ::libc::c_char) -> ();
    pub fn rb_mod_syserr_fail_str(arg1: VALUE, arg2: ::libc::c_int,
                                  arg3: VALUE) -> ();
    pub fn rb_warning(arg1: *const ::libc::c_char, ...) -> ();
    pub fn rb_compile_warning(arg1: *const ::libc::c_char,
                              arg2: ::libc::c_int,
                              arg3: *const ::libc::c_char, ...) -> ();
    pub fn rb_sys_warning(arg1: *const ::libc::c_char, ...) -> ();
    pub fn rb_warn(arg1: *const ::libc::c_char, ...) -> ();
    pub fn rb_compile_warn(arg1: *const ::libc::c_char, arg2: ::libc::c_int,
                           arg3: *const ::libc::c_char, ...) -> ();
    pub fn rb_each(arg1: VALUE) -> VALUE;
    pub fn rb_yield(arg1: VALUE) -> VALUE;
    pub fn rb_yield_values(n: ::libc::c_int, ...) -> VALUE;
    pub fn rb_yield_values2(n: ::libc::c_int, argv: *const VALUE) -> VALUE;
    pub fn rb_yield_splat(arg1: VALUE) -> VALUE;
    pub fn rb_yield_block(arg1: VALUE, arg2: VALUE, arg3: ::libc::c_int,
                          arg4: *const VALUE, arg5: VALUE) -> VALUE;
    pub fn rb_block_given_p() -> ::libc::c_int;
    pub fn rb_need_block() -> ();
    pub fn rb_iterate(arg1:
                          ::std::option::Option<extern "C" fn(arg1: VALUE)
                                                    -> VALUE>, arg2: VALUE,
                      arg3: ::std::option::Option<extern "C" fn() -> VALUE>,
                      arg4: VALUE) -> VALUE;
    pub fn rb_block_call(arg1: VALUE, arg2: ID, arg3: ::libc::c_int,
                         arg4: *const VALUE, arg5: rb_block_call_func_t,
                         arg6: VALUE) -> VALUE;
    pub fn rb_rescue(arg1: ::std::option::Option<extern "C" fn() -> VALUE>,
                     arg2: VALUE,
                     arg3: ::std::option::Option<extern "C" fn() -> VALUE>,
                     arg4: VALUE) -> VALUE;
    pub fn rb_rescue2(arg1: ::std::option::Option<extern "C" fn() -> VALUE>,
                      arg2: VALUE,
                      arg3: ::std::option::Option<extern "C" fn() -> VALUE>,
                      arg4: VALUE, ...) -> VALUE;
    pub fn rb_ensure(arg1: ::std::option::Option<extern "C" fn() -> VALUE>,
                     arg2: VALUE,
                     arg3: ::std::option::Option<extern "C" fn() -> VALUE>,
                     arg4: VALUE) -> VALUE;
    pub fn rb_catch(arg1: *const ::libc::c_char,
                    arg2: ::std::option::Option<extern "C" fn() -> VALUE>,
                    arg3: VALUE) -> VALUE;
    pub fn rb_catch_obj(arg1: VALUE,
                        arg2: ::std::option::Option<extern "C" fn() -> VALUE>,
                        arg3: VALUE) -> VALUE;
    pub fn rb_throw(arg1: *const ::libc::c_char, arg2: VALUE) -> ();
    pub fn rb_throw_obj(arg1: VALUE, arg2: VALUE) -> ();
    pub fn rb_require(arg1: *const ::libc::c_char) -> VALUE;
    pub fn ruby_native_thread_p() -> ::libc::c_int;
    pub fn rb_add_event_hook(func: rb_event_hook_func_t,
                             events: rb_event_flag_t, data: VALUE) -> ();
    pub fn rb_remove_event_hook(func: rb_event_hook_func_t) -> ::libc::c_int;
    pub fn rb_isalnum(c: ::libc::c_int) -> ::libc::c_int;
    pub fn rb_isalpha(c: ::libc::c_int) -> ::libc::c_int;
    pub fn rb_isblank(c: ::libc::c_int) -> ::libc::c_int;
    pub fn rb_iscntrl(c: ::libc::c_int) -> ::libc::c_int;
    pub fn rb_isdigit(c: ::libc::c_int) -> ::libc::c_int;
    pub fn rb_isgraph(c: ::libc::c_int) -> ::libc::c_int;
    pub fn rb_islower(c: ::libc::c_int) -> ::libc::c_int;
    pub fn rb_isprint(c: ::libc::c_int) -> ::libc::c_int;
    pub fn rb_ispunct(c: ::libc::c_int) -> ::libc::c_int;
    pub fn rb_isspace(c: ::libc::c_int) -> ::libc::c_int;
    pub fn rb_isupper(c: ::libc::c_int) -> ::libc::c_int;
    pub fn rb_isxdigit(c: ::libc::c_int) -> ::libc::c_int;
    pub fn rb_tolower(c: ::libc::c_int) -> ::libc::c_int;
    pub fn rb_toupper(c: ::libc::c_int) -> ::libc::c_int;
    pub fn st_locale_insensitive_strcasecmp(s1: *const ::libc::c_char,
                                            s2: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn st_locale_insensitive_strncasecmp(s1: *const ::libc::c_char,
                                             s2: *const ::libc::c_char,
                                             n: ::libc::size_t) -> ::libc::c_int;
    pub fn ruby_strtoul(str: *const ::libc::c_char,
                        endptr: *mut *mut ::libc::c_char, base: ::libc::c_int)
     -> ::libc::c_ulong;
    pub fn ruby_snprintf(str: *mut ::libc::c_char, n: ::libc::size_t,
                         fmt: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn ruby_vsnprintf(str: *mut ::libc::c_char, n: ::libc::size_t,
                          fmt: *const ::libc::c_char, ap: va_list)
     -> ::libc::c_int;
    pub fn ruby_sysinit(argc: *mut ::libc::c_int,
                        argv: *mut *mut *mut ::libc::c_char) -> ();
    pub fn ruby_init() -> ();
    pub fn ruby_options(argc: ::libc::c_int, argv: *mut *mut ::libc::c_char)
     -> *mut ::libc::c_void;
    pub fn ruby_executable_node(n: *mut ::libc::c_void,
                                status: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ruby_run_node(n: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn ruby_show_version() -> ();
    pub fn ruby_show_copyright() -> ();
    pub fn ruby_init_stack(arg1: *mut VALUE) -> ();
    pub fn ruby_setup() -> ::libc::c_int;
    pub fn ruby_cleanup(arg1: ::libc::c_int) -> ::libc::c_int;
    pub fn ruby_finalize() -> ();
    pub fn ruby_stop(arg1: ::libc::c_int) -> ();
    pub fn ruby_set_stack_size(arg1: ::libc::size_t) -> ();
    pub fn ruby_stack_check() -> ::libc::c_int;
    pub fn ruby_stack_length(arg1: *mut *mut VALUE) -> ::libc::size_t;
    pub fn ruby_exec_node(n: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn ruby_script(name: *const ::libc::c_char) -> ();
    pub fn ruby_set_script_name(name: VALUE) -> ();
    pub fn ruby_prog_init() -> ();
    pub fn ruby_set_argv(arg1: ::libc::c_int, arg2: *mut *mut ::libc::c_char)
     -> ();
    pub fn ruby_process_options(arg1: ::libc::c_int,
                                arg2: *mut *mut ::libc::c_char)
     -> *mut ::libc::c_void;
    pub fn ruby_init_loadpath() -> ();
    pub fn ruby_incpush(arg1: *const ::libc::c_char) -> ();
    pub fn ruby_sig_finalize() -> ();
}
