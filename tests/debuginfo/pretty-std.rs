// ignore-tidy-linelength
//@ ignore-freebsd: gdb package too new
//@ only-cdb // "Temporarily" ignored on GDB/LLDB due to debuginfo tests being disabled, see PR 47155
//@ ignore-android: FIXME(#10381)
//@ compile-flags:-g
//@ min-gdb-version: 7.7
//@ min-lldb-version: 310
//@ min-cdb-version: 10.0.18317.1001

// === GDB TESTS ===================================================================================

// gdb-command: run

// gdb-command: print slice
// gdb-check:$1 = &[i32](len: 4) = {0, 1, 2, 3}

// gdb-command: print vec
// gdb-check:$2 = Vec<u64, alloc::alloc::Global>(len: 4, cap: [...]) = {4, 5, 6, 7}

// gdb-command: print str_slice
// gdb-check:$3 = "IAMA string slice!"

// gdb-command: print string
// gdb-check:$4 = "IAMA string!"

// gdb-command: print some
// gdb-check:$5 = Some = {8}

// gdb-command: print none
// gdbg-check:$6 = None
// gdbr-check:$6 = core::option::Option::None

// gdb-command: print os_string
// gdb-check:$7 = "IAMA OS string 😃"

// gdb-command: print some_string
// gdb-check:$8 = Some = {"IAMA optional string!"}

// gdb-command: set print length 5
// gdb-command: print some_string
// gdb-check:$8 = Some = {"IAMA "...}

// === LLDB TESTS ==================================================================================

// lldb-command:run

// lldb-command:v slice
// lldb-check:[...] slice = &[0, 1, 2, 3]

// lldb-command:v vec
// lldb-check:[...] vec = vec![4, 5, 6, 7]

// lldb-command:v str_slice
// lldb-check:[...] str_slice = "IAMA string slice!"

// lldb-command:v string
// lldb-check:[...] string = "IAMA string!"

// lldb-command:v some
// lldb-check:[...] some = Some(8)

// lldb-command:v none
// lldb-check:[...] none = None

// lldb-command:v os_string
// lldb-check:[...] os_string = "IAMA OS string 😃"[...]

// === CDB TESTS ==================================================================================

// cdb-command: g

// cdb-command: dx slice,d
// cdb-check:slice,d          : { len=4 } [Type: ref$<slice2$<i32> >]
// cdb-check:    [len]            : 4 [Type: [...]]
// cdb-check:    [0]              : 0 [Type: int]
// cdb-check:    [1]              : 1 [Type: int]
// cdb-check:    [2]              : 2 [Type: int]
// cdb-check:    [3]              : 3 [Type: int]

// cdb-command: dx vec,d
// cdb-check:vec,d [...] : { len=4 } [Type: [...]::Vec<u64,alloc::alloc::Global>]
// cdb-check:    [len]            : 4 [Type: [...]]
// cdb-check:    [capacity]       : [...] [Type: [...]]
// cdb-check:    [0]              : 4 [Type: u64]
// cdb-check:    [1]              : 5 [Type: u64]
// cdb-check:    [2]              : 6 [Type: u64]
// cdb-check:    [3]              : 7 [Type: u64]

// cdb-command: dx str_slice
// cdb-check:str_slice        : "IAMA string slice!" [Type: ref$<str$>]

// cdb-command: dx string
// cdb-check:string           : "IAMA string!" [Type: [...]::String]
// cdb-check:    [<Raw View>]     [Type: [...]::String]
// cdb-check:    [len]            : 0xc [Type: [...]]
// cdb-check:    [capacity]       : 0xc [Type: [...]]

// cdb-command: dx -r2 string
// cdb-check:    [0]              : 73 'I' [Type: char]
// cdb-check:    [1]              : 65 'A' [Type: char]
// cdb-check:    [2]              : 77 'M' [Type: char]
// cdb-check:    [3]              : 65 'A' [Type: char]
// cdb-check:    [4]              : 32 ' ' [Type: char]
// cdb-check:    [5]              : 115 's' [Type: char]
// cdb-check:    [6]              : 116 't' [Type: char]
// cdb-check:    [7]              : 114 'r' [Type: char]
// cdb-check:    [8]              : 105 'i' [Type: char]
// cdb-check:    [9]              : 110 'n' [Type: char]
// cdb-check:    [10]             : 103 'g' [Type: char]
// cdb-check:    [11]             : 33 '!' [Type: char]

// cdb-command: dx os_string
// NOTE: OSString is WTF-8 encoded which Windows debuggers don't understand. Verify the UTF-8
//       portion displays correctly.
// cdb-check:os_string        : "IAMA OS string [...]" [Type: std::ffi::os_str::OsString]
// cdb-check:    [<Raw View>]     [Type: std::ffi::os_str::OsString]
// cdb-check:    [chars]          : "IAMA OS string [...]"

// cdb-command: dx some
// cdb-check:some             : Some [Type: enum2$<core::option::Option<i16> >]
// cdb-check:    [<Raw View>]     [Type: enum2$<core::option::Option<i16> >]
// cdb-check:    [+0x002] __0              : 8 [Type: short]

// cdb-command: dx none
// cdb-check:none             : None [Type: enum2$<core::option::Option<i64> >]
// cdb-check:    [<Raw View>]     [Type: enum2$<core::option::Option<i64> >]

// cdb-command: dx some_string
// cdb-check:some_string      : Some [Type: enum2$<core::option::Option<alloc::string::String> >]
// cdb-check:    [<Raw View>]     [Type: enum2$<core::option::Option<alloc::string::String> >]
// cdb-check:    [+0x000] __0              : "IAMA optional string!" [Type: alloc::string::String]

// cdb-command: dx linkedlist
// cdb-check:linkedlist       : { len=0x2 } [Type: alloc::collections::linked_list::LinkedList<i32,alloc::alloc::Global>]
// cdb-check:    [<Raw View>]     [Type: alloc::collections::linked_list::LinkedList<i32,alloc::alloc::Global>]
// cdb-check:    [0x0]            : 128 [Type: int]
// cdb-check:    [0x1]            : 42 [Type: int]

// cdb-command: dx vecdeque
// cdb-check:vecdeque         : { len=0x2 } [Type: alloc::collections::vec_deque::VecDeque<i32,alloc::alloc::Global>]
// cdb-check:    [<Raw View>]     [Type: alloc::collections::vec_deque::VecDeque<i32,alloc::alloc::Global>]
// cdb-check:    [len]            : 0x2 [Type: unsigned [...]]
// cdb-check:    [capacity]       : 0x8 [Type: unsigned [...]]
// cdb-check:    [0x0]            : 90 [Type: i32]
// cdb-check:    [0x1]            : 20 [Type: i32]

#![allow(unused_variables)]
use std::collections::{LinkedList, VecDeque};
use std::ffi::OsString;

fn main() {
    // &[]
    let slice: &[i32] = &[0, 1, 2, 3];

    // Vec
    let vec = vec![4u64, 5, 6, 7];

    // &str
    let str_slice = "IAMA string slice!";

    // String
    let string = "IAMA string!".to_string();

    // OsString
    let os_string = OsString::from("IAMA OS string \u{1F603}");

    // Option
    let some = Some(8i16);
    let none: Option<i64> = None;

    let some_string = Some("IAMA optional string!".to_owned());

    // LinkedList
    let mut linkedlist = LinkedList::new();
    linkedlist.push_back(42);
    linkedlist.push_front(128);

    // VecDeque
    let mut vecdeque = VecDeque::with_capacity(8);
    vecdeque.push_back(20);
    vecdeque.push_front(90);

    zzz(); // #break
}

fn zzz() {
    ()
}
