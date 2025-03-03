// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

module rooch_examples::complex_struct {

   use moveos_std::context::{Self, Context};
   use moveos_std::account_storage;
   use moveos_std::object::{Self, Object};
   use moveos_std::object::ObjectID;
   use moveos_std::bcs;
   use moveos_std::signer;
   use std::vector;

   struct SimpleStruct has store, copy, drop {
      value: u64,
   }

   struct ComplexStruct has key, store {
      field_u8: u8,
      field_u16: u16,
      field_u32: u32,
      field_u64: u64,
      field_u128: u128,
      field_u256: u256,
      field_address: address,
      field_object_id: ObjectID,
      field_ascii_str: std::ascii::String,
      field_str: std::string::String,
      field_struct: SimpleStruct,
      field_vec_u8: vector<u8>,
      field_vec_u16: vector<u16>,
      field_vec_u32: vector<u32>,
      field_vec_u64: vector<u64>,
      field_vec_str: vector<std::string::String>,
      field_vec_struct: vector<SimpleStruct>,
   }

   fun new_complex_struct(object_id: ObjectID): ComplexStruct{

      let simple_struct = SimpleStruct{ value: 42};

      let vec_u16 = vector::empty<u16>();
      vector::push_back(&mut vec_u16, 1);
      vector::push_back(&mut vec_u16, 0xFFFF);

      let vec_u32 = vector::empty<u32>();
      vector::push_back(&mut vec_u32, 1);
      vector::push_back(&mut vec_u32, 0xFFFFFFFF);

      let vec_u64 = vector::empty<u64>();
      vector::push_back(&mut vec_u64, 1);
      vector::push_back(&mut vec_u64, 0xFFFFFFFFFFFFFFFF);

      //e4bda0e5a5bd is Chinese nihao
      let utf8_str = std::string::utf8(x"e4bda0e5a5bd");

      let vec_str = vector::empty<std::string::String>();
      vector::push_back(&mut vec_str, std::string::utf8(b"hello"));
      vector::push_back(&mut vec_str, copy utf8_str);

      let vec_struct = vector::empty<SimpleStruct>();
      vector::push_back(&mut vec_struct, *&simple_struct);

      ComplexStruct {
         field_u8: 0xFF,
         field_u16: 0xFFFF,
         field_u32: 0xFFFFFFFF,
         field_u64: 0xFFFFFFFFFFFFFFFF,
         field_u128: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
         field_u256: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
         field_address: @rooch_examples,
         field_object_id: object_id,
         field_ascii_str: std::ascii::string(b"hello"),
         field_str: utf8_str,
         field_struct: simple_struct,
         field_vec_u8: bcs::to_bytes(&@rooch_examples),
         field_vec_u16: vec_u16, 
         field_vec_u32: vec_u32, 
         field_vec_u64: vec_u64, 
         field_vec_str: vec_str,
         field_vec_struct: vec_struct,  
      }
   }

   fun drop(s: ComplexStruct){
      let ComplexStruct{
         field_u8: _,
         field_u16: _,
         field_u32: _,
         field_u64: _,
         field_u128: _,
         field_u256: _,
         field_address: _,
         field_object_id: _,
         field_ascii_str: _,
         field_str: _,
         field_struct: _,
         field_vec_u8: _,
         field_vec_u16: _,
         field_vec_u32: _,
         field_vec_u64: _,
         field_vec_str: _,
         field_vec_struct: _,
      } = s;
   } 

   //init when module publish
   fun init(ctx: &mut Context) {
      let module_signer = signer::module_signer<ComplexStruct>();
      let object_id = context::fresh_object_id(ctx);
      let s = new_complex_struct(object_id);
      let complex_object = context::new_object(ctx, s);
      object::to_permanent(complex_object);
      let s2 = new_complex_struct(object_id);
      account_storage::global_move_to(ctx, &module_signer, s2);
   }

   public fun value(ctx: &Context): &ComplexStruct {
      account_storage::global_borrow<ComplexStruct>(ctx,@rooch_examples)
   }

   public fun value_of_object(obj: &Object<ComplexStruct>) : &ComplexStruct {
      object::borrow(obj)
   }
}
