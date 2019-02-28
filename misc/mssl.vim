" Vim syntax file
" Language: My Simple Scripting Language
" Maintainer: Wojciech Bartnik <yisonPylkita@gmail.com>
" Latest Revision: 28 February 2019

if exists("b:current_syntax")
  finish
endif

" Keywords
syn keyword mssl_keywords fn return let loop break in for continue match const if else struct self true false 
syn keyword mssl_todo contained TODO FIXME NOTE
syn keyword mssl_basic_types i8 u8 i32 u32 i64 u64 f32 f64

" Matches
syn match mssl_comment "#.*$" contains=mssl_todo
syn match mssl_number '\d\+'
syn match mssl_number '[-+]\d\+'

" Regions
syn region scope start="{" end="}" fold transparent
syn region mssl_string start='"' end='"' contained
syn region mssl_desc start='"' end='"'
syn region mssl_desc start='\'' end='\''

let b:current_syntax = "mssl"
hi def link mssl_todo        Todo
hi def link mssl_comment     Comment
hi def link mssl_keywords    Statement
hi def link mssl_string      Constant
hi def link mssl_desc        PreProc
hi def link mssl_number      Number
hi def link mssl_basic_types Type

