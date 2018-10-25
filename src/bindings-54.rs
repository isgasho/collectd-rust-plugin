/* automatically generated by rust-bindgen */

pub const OCONFIG_TYPE_STRING: u32 = 0;
pub const OCONFIG_TYPE_NUMBER: u32 = 1;
pub const OCONFIG_TYPE_BOOLEAN: u32 = 2;
pub const DATA_MAX_NAME_LEN: u32 = 64;
pub const DS_TYPE_COUNTER: u32 = 0;
pub const DS_TYPE_GAUGE: u32 = 1;
pub const DS_TYPE_DERIVE: u32 = 2;
pub const DS_TYPE_ABSOLUTE: u32 = 3;
pub const LOG_ERR: u32 = 3;
pub const LOG_WARNING: u32 = 4;
pub const LOG_NOTICE: u32 = 5;
pub const LOG_INFO: u32 = 6;
pub const LOG_DEBUG: u32 = 7;
pub type __time_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[test]
fn bindgen_test_layout_timespec() {
    assert_eq!(
        ::std::mem::size_of::<timespec>(),
        16usize,
        concat!("Size of: ", stringify!(timespec))
    );
    assert_eq!(
        ::std::mem::align_of::<timespec>(),
        8usize,
        concat!("Alignment of ", stringify!(timespec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timespec>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timespec>())).tv_nsec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_nsec)
        )
    );
}
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[test]
fn bindgen_test_layout_pthread_attr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_attr_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_attr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_attr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_attr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type cdtime_t = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oconfig_value_s {
    pub value: oconfig_value_s__bindgen_ty_1,
    pub type_: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union oconfig_value_s__bindgen_ty_1 {
    pub string: *mut ::std::os::raw::c_char,
    pub number: f64,
    pub boolean: ::std::os::raw::c_int,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_oconfig_value_s__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<oconfig_value_s__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(oconfig_value_s__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<oconfig_value_s__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(oconfig_value_s__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<oconfig_value_s__bindgen_ty_1>())).string as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(oconfig_value_s__bindgen_ty_1),
            "::",
            stringify!(string)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<oconfig_value_s__bindgen_ty_1>())).number as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(oconfig_value_s__bindgen_ty_1),
            "::",
            stringify!(number)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<oconfig_value_s__bindgen_ty_1>())).boolean as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(oconfig_value_s__bindgen_ty_1),
            "::",
            stringify!(boolean)
        )
    );
}
#[test]
fn bindgen_test_layout_oconfig_value_s() {
    assert_eq!(
        ::std::mem::size_of::<oconfig_value_s>(),
        16usize,
        concat!("Size of: ", stringify!(oconfig_value_s))
    );
    assert_eq!(
        ::std::mem::align_of::<oconfig_value_s>(),
        8usize,
        concat!("Alignment of ", stringify!(oconfig_value_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<oconfig_value_s>())).value as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(oconfig_value_s),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<oconfig_value_s>())).type_ as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(oconfig_value_s),
            "::",
            stringify!(type_)
        )
    );
}
pub type oconfig_value_t = oconfig_value_s;
pub type oconfig_item_t = oconfig_item_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct oconfig_item_s {
    pub key: *mut ::std::os::raw::c_char,
    pub values: *mut oconfig_value_t,
    pub values_num: ::std::os::raw::c_int,
    pub parent: *mut oconfig_item_t,
    pub children: *mut oconfig_item_t,
    pub children_num: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_oconfig_item_s() {
    assert_eq!(
        ::std::mem::size_of::<oconfig_item_s>(),
        48usize,
        concat!("Size of: ", stringify!(oconfig_item_s))
    );
    assert_eq!(
        ::std::mem::align_of::<oconfig_item_s>(),
        8usize,
        concat!("Alignment of ", stringify!(oconfig_item_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<oconfig_item_s>())).key as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(oconfig_item_s),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<oconfig_item_s>())).values as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(oconfig_item_s),
            "::",
            stringify!(values)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<oconfig_item_s>())).values_num as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(oconfig_item_s),
            "::",
            stringify!(values_num)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<oconfig_item_s>())).parent as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(oconfig_item_s),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<oconfig_item_s>())).children as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(oconfig_item_s),
            "::",
            stringify!(children)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<oconfig_item_s>())).children_num as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(oconfig_item_s),
            "::",
            stringify!(children_num)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct meta_data_s {
    _unused: [u8; 0],
}
pub type meta_data_t = meta_data_s;
pub type counter_t = ::std::os::raw::c_ulonglong;
pub type gauge_t = f64;
pub type derive_t = i64;
pub type absolute_t = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub union value_u {
    pub counter: counter_t,
    pub gauge: gauge_t,
    pub derive: derive_t,
    pub absolute: absolute_t,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_value_u() {
    assert_eq!(
        ::std::mem::size_of::<value_u>(),
        8usize,
        concat!("Size of: ", stringify!(value_u))
    );
    assert_eq!(
        ::std::mem::align_of::<value_u>(),
        8usize,
        concat!("Alignment of ", stringify!(value_u))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_u>())).counter as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(value_u),
            "::",
            stringify!(counter)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_u>())).gauge as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(value_u),
            "::",
            stringify!(gauge)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_u>())).derive as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(value_u),
            "::",
            stringify!(derive)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_u>())).absolute as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(value_u),
            "::",
            stringify!(absolute)
        )
    );
}
pub type value_t = value_u;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct value_list_s {
    pub values: *mut value_t,
    pub values_len: ::std::os::raw::c_int,
    pub time: cdtime_t,
    pub interval: cdtime_t,
    pub host: [::std::os::raw::c_char; 64usize],
    pub plugin: [::std::os::raw::c_char; 64usize],
    pub plugin_instance: [::std::os::raw::c_char; 64usize],
    pub type_: [::std::os::raw::c_char; 64usize],
    pub type_instance: [::std::os::raw::c_char; 64usize],
    pub meta: *mut meta_data_t,
}
#[test]
fn bindgen_test_layout_value_list_s() {
    assert_eq!(
        ::std::mem::size_of::<value_list_s>(),
        360usize,
        concat!("Size of: ", stringify!(value_list_s))
    );
    assert_eq!(
        ::std::mem::align_of::<value_list_s>(),
        8usize,
        concat!("Alignment of ", stringify!(value_list_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_list_s>())).values as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(value_list_s),
            "::",
            stringify!(values)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_list_s>())).values_len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(value_list_s),
            "::",
            stringify!(values_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_list_s>())).time as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(value_list_s),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_list_s>())).interval as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(value_list_s),
            "::",
            stringify!(interval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_list_s>())).host as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(value_list_s),
            "::",
            stringify!(host)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_list_s>())).plugin as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(value_list_s),
            "::",
            stringify!(plugin)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_list_s>())).plugin_instance as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(value_list_s),
            "::",
            stringify!(plugin_instance)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_list_s>())).type_ as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(value_list_s),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_list_s>())).type_instance as *const _ as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(value_list_s),
            "::",
            stringify!(type_instance)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value_list_s>())).meta as *const _ as usize },
        352usize,
        concat!(
            "Offset of field: ",
            stringify!(value_list_s),
            "::",
            stringify!(meta)
        )
    );
}
pub type value_list_t = value_list_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_source_s {
    pub name: [::std::os::raw::c_char; 64usize],
    pub type_: ::std::os::raw::c_int,
    pub min: f64,
    pub max: f64,
}
#[test]
fn bindgen_test_layout_data_source_s() {
    assert_eq!(
        ::std::mem::size_of::<data_source_s>(),
        88usize,
        concat!("Size of: ", stringify!(data_source_s))
    );
    assert_eq!(
        ::std::mem::align_of::<data_source_s>(),
        8usize,
        concat!("Alignment of ", stringify!(data_source_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<data_source_s>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(data_source_s),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<data_source_s>())).type_ as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(data_source_s),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<data_source_s>())).min as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(data_source_s),
            "::",
            stringify!(min)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<data_source_s>())).max as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(data_source_s),
            "::",
            stringify!(max)
        )
    );
}
pub type data_source_t = data_source_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_set_s {
    pub type_: [::std::os::raw::c_char; 64usize],
    pub ds_num: ::std::os::raw::c_int,
    pub ds: *mut data_source_t,
}
#[test]
fn bindgen_test_layout_data_set_s() {
    assert_eq!(
        ::std::mem::size_of::<data_set_s>(),
        80usize,
        concat!("Size of: ", stringify!(data_set_s))
    );
    assert_eq!(
        ::std::mem::align_of::<data_set_s>(),
        8usize,
        concat!("Alignment of ", stringify!(data_set_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<data_set_s>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(data_set_s),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<data_set_s>())).ds_num as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(data_set_s),
            "::",
            stringify!(ds_num)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<data_set_s>())).ds as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(data_set_s),
            "::",
            stringify!(ds)
        )
    );
}
pub type data_set_t = data_set_s;
pub const notification_meta_type_e_NM_TYPE_STRING: notification_meta_type_e = 0;
pub const notification_meta_type_e_NM_TYPE_SIGNED_INT: notification_meta_type_e = 1;
pub const notification_meta_type_e_NM_TYPE_UNSIGNED_INT: notification_meta_type_e = 2;
pub const notification_meta_type_e_NM_TYPE_DOUBLE: notification_meta_type_e = 3;
pub const notification_meta_type_e_NM_TYPE_BOOLEAN: notification_meta_type_e = 4;
pub type notification_meta_type_e = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notification_meta_s {
    pub name: [::std::os::raw::c_char; 64usize],
    pub type_: notification_meta_type_e,
    pub nm_value: notification_meta_s__bindgen_ty_1,
    pub next: *mut notification_meta_s,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union notification_meta_s__bindgen_ty_1 {
    pub nm_string: *const ::std::os::raw::c_char,
    pub nm_signed_int: i64,
    pub nm_unsigned_int: u64,
    pub nm_double: f64,
    pub nm_boolean: bool,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_notification_meta_s__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<notification_meta_s__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(notification_meta_s__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<notification_meta_s__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(notification_meta_s__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<notification_meta_s__bindgen_ty_1>())).nm_string as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_meta_s__bindgen_ty_1),
            "::",
            stringify!(nm_string)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<notification_meta_s__bindgen_ty_1>())).nm_signed_int as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_meta_s__bindgen_ty_1),
            "::",
            stringify!(nm_signed_int)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<notification_meta_s__bindgen_ty_1>())).nm_unsigned_int
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_meta_s__bindgen_ty_1),
            "::",
            stringify!(nm_unsigned_int)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<notification_meta_s__bindgen_ty_1>())).nm_double as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_meta_s__bindgen_ty_1),
            "::",
            stringify!(nm_double)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<notification_meta_s__bindgen_ty_1>())).nm_boolean as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_meta_s__bindgen_ty_1),
            "::",
            stringify!(nm_boolean)
        )
    );
}
#[test]
fn bindgen_test_layout_notification_meta_s() {
    assert_eq!(
        ::std::mem::size_of::<notification_meta_s>(),
        88usize,
        concat!("Size of: ", stringify!(notification_meta_s))
    );
    assert_eq!(
        ::std::mem::align_of::<notification_meta_s>(),
        8usize,
        concat!("Alignment of ", stringify!(notification_meta_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<notification_meta_s>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_meta_s),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<notification_meta_s>())).type_ as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_meta_s),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<notification_meta_s>())).nm_value as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_meta_s),
            "::",
            stringify!(nm_value)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<notification_meta_s>())).next as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_meta_s),
            "::",
            stringify!(next)
        )
    );
}
pub type notification_meta_t = notification_meta_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notification_s {
    pub severity: ::std::os::raw::c_int,
    pub time: cdtime_t,
    pub message: [::std::os::raw::c_char; 256usize],
    pub host: [::std::os::raw::c_char; 64usize],
    pub plugin: [::std::os::raw::c_char; 64usize],
    pub plugin_instance: [::std::os::raw::c_char; 64usize],
    pub type_: [::std::os::raw::c_char; 64usize],
    pub type_instance: [::std::os::raw::c_char; 64usize],
    pub meta: *mut notification_meta_t,
}
#[test]
fn bindgen_test_layout_notification_s() {
    assert_eq!(
        ::std::mem::size_of::<notification_s>(),
        600usize,
        concat!("Size of: ", stringify!(notification_s))
    );
    assert_eq!(
        ::std::mem::align_of::<notification_s>(),
        8usize,
        concat!("Alignment of ", stringify!(notification_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<notification_s>())).severity as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_s),
            "::",
            stringify!(severity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<notification_s>())).time as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_s),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<notification_s>())).message as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_s),
            "::",
            stringify!(message)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<notification_s>())).host as *const _ as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_s),
            "::",
            stringify!(host)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<notification_s>())).plugin as *const _ as usize },
        336usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_s),
            "::",
            stringify!(plugin)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<notification_s>())).plugin_instance as *const _ as usize },
        400usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_s),
            "::",
            stringify!(plugin_instance)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<notification_s>())).type_ as *const _ as usize },
        464usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_s),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<notification_s>())).type_instance as *const _ as usize },
        528usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_s),
            "::",
            stringify!(type_instance)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<notification_s>())).meta as *const _ as usize },
        592usize,
        concat!(
            "Offset of field: ",
            stringify!(notification_s),
            "::",
            stringify!(meta)
        )
    );
}
pub type notification_t = notification_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct user_data_s {
    pub data: *mut ::std::os::raw::c_void,
    pub free_func: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
}
#[test]
fn bindgen_test_layout_user_data_s() {
    assert_eq!(
        ::std::mem::size_of::<user_data_s>(),
        16usize,
        concat!("Size of: ", stringify!(user_data_s))
    );
    assert_eq!(
        ::std::mem::align_of::<user_data_s>(),
        8usize,
        concat!("Alignment of ", stringify!(user_data_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<user_data_s>())).data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(user_data_s),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<user_data_s>())).free_func as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(user_data_s),
            "::",
            stringify!(free_func)
        )
    );
}
pub type user_data_t = user_data_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct plugin_ctx_s {
    pub interval: cdtime_t,
}
#[test]
fn bindgen_test_layout_plugin_ctx_s() {
    assert_eq!(
        ::std::mem::size_of::<plugin_ctx_s>(),
        8usize,
        concat!("Size of: ", stringify!(plugin_ctx_s))
    );
    assert_eq!(
        ::std::mem::align_of::<plugin_ctx_s>(),
        8usize,
        concat!("Alignment of ", stringify!(plugin_ctx_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<plugin_ctx_s>())).interval as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(plugin_ctx_s),
            "::",
            stringify!(interval)
        )
    );
}
pub type plugin_ctx_t = plugin_ctx_s;
pub type plugin_init_cb = ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>;
pub type plugin_read_cb =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut user_data_t) -> ::std::os::raw::c_int>;
pub type plugin_write_cb = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *const data_set_t,
        arg2: *const value_list_t,
        arg3: *mut user_data_t,
    ) -> ::std::os::raw::c_int,
>;
pub type plugin_flush_cb = ::std::option::Option<
    unsafe extern "C" fn(
        timeout: cdtime_t,
        identifier: *const ::std::os::raw::c_char,
        arg1: *mut user_data_t,
    ) -> ::std::os::raw::c_int,
>;
pub type plugin_missing_cb = ::std::option::Option<
    unsafe extern "C" fn(arg1: *const value_list_t, arg2: *mut user_data_t)
        -> ::std::os::raw::c_int,
>;
pub type plugin_log_cb = ::std::option::Option<
    unsafe extern "C" fn(
        severity: ::std::os::raw::c_int,
        message: *const ::std::os::raw::c_char,
        arg1: *mut user_data_t,
    ),
>;
pub type plugin_shutdown_cb =
    ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>;
pub type plugin_notification_cb = ::std::option::Option<
    unsafe extern "C" fn(arg1: *const notification_t, arg2: *mut user_data_t)
        -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn plugin_set_dir(dir: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn plugin_load(name: *const ::std::os::raw::c_char, flags: u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_init_all();
}
extern "C" {
    pub fn plugin_read_all();
}
extern "C" {
    pub fn plugin_read_all_once() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_shutdown_all();
}
extern "C" {
    pub fn plugin_write(
        plugin: *const ::std::os::raw::c_char,
        ds: *const data_set_t,
        vl: *const value_list_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_flush(
        plugin: *const ::std::os::raw::c_char,
        timeout: cdtime_t,
        identifier: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_register_config(
        name: *const ::std::os::raw::c_char,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                key: *const ::std::os::raw::c_char,
                val: *const ::std::os::raw::c_char,
            ) -> ::std::os::raw::c_int,
        >,
        keys: *mut *const ::std::os::raw::c_char,
        keys_num: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_register_complex_config(
        type_: *const ::std::os::raw::c_char,
        callback: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut oconfig_item_t) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_register_init(
        name: *const ::std::os::raw::c_char,
        callback: plugin_init_cb,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_register_read(
        name: *const ::std::os::raw::c_char,
        callback: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_register_complex_read(
        group: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        callback: plugin_read_cb,
        interval: *const timespec,
        user_data: *mut user_data_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_register_write(
        name: *const ::std::os::raw::c_char,
        callback: plugin_write_cb,
        user_data: *mut user_data_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_register_flush(
        name: *const ::std::os::raw::c_char,
        callback: plugin_flush_cb,
        user_data: *mut user_data_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_register_missing(
        name: *const ::std::os::raw::c_char,
        callback: plugin_missing_cb,
        user_data: *mut user_data_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_register_shutdown(
        name: *const ::std::os::raw::c_char,
        callback: plugin_shutdown_cb,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_register_data_set(ds: *const data_set_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_register_log(
        name: *const ::std::os::raw::c_char,
        callback: plugin_log_cb,
        user_data: *mut user_data_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_register_notification(
        name: *const ::std::os::raw::c_char,
        callback: plugin_notification_cb,
        user_data: *mut user_data_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_unregister_config(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_unregister_complex_config(
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_unregister_init(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_unregister_read(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_unregister_read_group(
        group: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_unregister_write(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_unregister_flush(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_unregister_missing(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_unregister_shutdown(name: *const ::std::os::raw::c_char)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_unregister_data_set(name: *const ::std::os::raw::c_char)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_unregister_log(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_unregister_notification(
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_dispatch_values(vl: *const value_list_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_dispatch_missing(vl: *const value_list_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_dispatch_notification(notif: *const notification_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_log(level: ::std::os::raw::c_int, format: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn plugin_get_ds(name: *const ::std::os::raw::c_char) -> *const data_set_t;
}
extern "C" {
    pub fn plugin_notification_meta_add_string(
        n: *mut notification_t,
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_notification_meta_add_signed_int(
        n: *mut notification_t,
        name: *const ::std::os::raw::c_char,
        value: i64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_notification_meta_add_unsigned_int(
        n: *mut notification_t,
        name: *const ::std::os::raw::c_char,
        value: u64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_notification_meta_add_double(
        n: *mut notification_t,
        name: *const ::std::os::raw::c_char,
        value: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_notification_meta_add_boolean(
        n: *mut notification_t,
        name: *const ::std::os::raw::c_char,
        value: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_notification_meta_copy(
        dst: *mut notification_t,
        src: *const notification_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_notification_meta_free(n: *mut notification_meta_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn plugin_init_ctx();
}
extern "C" {
    pub fn plugin_get_ctx() -> plugin_ctx_t;
}
extern "C" {
    pub fn plugin_set_ctx(ctx: plugin_ctx_t) -> plugin_ctx_t;
}
extern "C" {
    pub fn plugin_get_interval() -> cdtime_t;
}
extern "C" {
    pub fn plugin_thread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
        >,
        arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn uc_get_rate(ds: *const data_set_t, vl: *const value_list_t) -> *mut gauge_t;
}
