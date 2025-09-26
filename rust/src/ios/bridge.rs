use std::slice::from_raw_parts;
use super::bridge_tools::result::*;
use super::bridge_tools::string::*;
use super::bridge_tools::data::*;
use crate::js_result::*;
use crate::panic::*;
use crate::ptr::*;
use crate::enum_maps::*;
use crate::arrays::*;
use cardano_message_signing::COSEEncrypt0;
use cardano_message_signing::COSEEncrypt;
use cardano_message_signing::COSEKey;
use cardano_message_signing::COSERecipient;
use cardano_message_signing::COSERecipients;
use cardano_message_signing::COSESign1;
use cardano_message_signing::COSESign;
use cardano_message_signing::COSESignature;
use cardano_message_signing::COSESignatures;
use cardano_message_signing::CounterSignature;
use cardano_message_signing::HeaderMap;
use cardano_message_signing::Headers;
use cardano_message_signing::Label;
use cardano_message_signing::LabelKind;
use cardano_message_signing::Labels;
use cardano_message_signing::PasswordEncryption;
use cardano_message_signing::ProtectedHeaderMap;
use cardano_message_signing::PubKeyEncryption;
use cardano_message_signing::SigContext;
use cardano_message_signing::SigStructure;
use cardano_message_signing::SignedMessage;
use cardano_message_signing::SignedMessageKind;
use cardano_message_signing::builders::AlgorithmId;
use cardano_message_signing::builders::COSESign1Builder;
use cardano_message_signing::builders::COSESignBuilder;
use cardano_message_signing::builders::CurveType;
use cardano_message_signing::builders::ECKey;
use cardano_message_signing::builders::EdDSA25519Key;
use cardano_message_signing::builders::KeyOperation;
use cardano_message_signing::builders::KeyType;
use cardano_message_signing::cbor::CBORArray;
use cardano_message_signing::cbor::CBORObject;
use cardano_message_signing::cbor::CBORSpecial;
use cardano_message_signing::cbor::CBORSpecialType;
use cardano_message_signing::cbor::CBORValue;
use cardano_message_signing::cbor::CBORValueKind;
use cardano_message_signing::cbor::TaggedCBOR;
use cardano_message_signing::utils::BigNum;
use cardano_message_signing::utils::Int;
use cardano_message_signing::utils::FromBytes;
use cardano_message_signing::utils::ToBytes;


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_big_num_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<BigNum>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_big_num_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = BigNum::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_big_num_from_str(string_str: CharPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let string: &str = string_str.into_str();
    let result = BigNum::from_str(string).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_big_num_to_str(self_rptr: RPtr, result: &mut CharPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<BigNum>()?;
    let result = self_ref.to_str();
    Ok::<CharPtr, String>(result.into_cstr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_big_num_checked_mul(self_rptr: RPtr, other_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<BigNum>()?;
    let other = other_rptr.typed_ref::<BigNum>()?;
    let result = self_ref.checked_mul(other).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_big_num_checked_add(self_rptr: RPtr, other_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<BigNum>()?;
    let other = other_rptr.typed_ref::<BigNum>()?;
    let result = self_ref.checked_add(other).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_big_num_checked_sub(self_rptr: RPtr, other_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<BigNum>()?;
    let other = other_rptr.typed_ref::<BigNum>()?;
    let result = self_ref.checked_sub(other).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_array_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORArray>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_array_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = CBORArray::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_array_new(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let result = CBORArray::new();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_array_len(self_rptr: RPtr, result: &mut i64, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORArray>()?;
    let result = self_ref.len();
    Ok::<i64, String>(result as i64)
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_array_get(self_rptr: RPtr, index_long: i64, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORArray>()?;
    let index  = index_long as usize;
    let result = self_ref.get(index);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_array_add(self_rptr: RPtr, elem_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORArray>()?;
    let elem = elem_rptr.typed_ref::<CBORValue>()?;
    self_ref.add(elem);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_array_set_definite_encoding(self_rptr: RPtr, use_definite: bool, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORArray>()?;
    self_ref.set_definite_encoding(use_definite);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_array_is_definite(self_rptr: RPtr, result: &mut bool, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORArray>()?;
    let result = self_ref.is_definite();
    Ok::<bool, String>(result)
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_object_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORObject>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_object_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = CBORObject::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_object_new(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let result = CBORObject::new();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_object_len(self_rptr: RPtr, result: &mut i64, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORObject>()?;
    let result = self_ref.len();
    Ok::<i64, String>(result as i64)
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_object_insert(self_rptr: RPtr, key_rptr: RPtr, value_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORObject>()?;
    let key = key_rptr.typed_ref::<CBORValue>()?;
    let value = value_rptr.typed_ref::<CBORValue>()?;
    let result = self_ref.insert(key, value);
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_object_get(self_rptr: RPtr, key_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORObject>()?;
    let key = key_rptr.typed_ref::<CBORValue>()?;
    let result = self_ref.get(key);
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_object_keys(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORObject>()?;
    let result = self_ref.keys();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_object_set_definite_encoding(self_rptr: RPtr, use_definite: bool, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORObject>()?;
    self_ref.set_definite_encoding(use_definite);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_object_is_definite(self_rptr: RPtr, result: &mut bool, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORObject>()?;
    let result = self_ref.is_definite();
    Ok::<bool, String>(result)
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_special_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORSpecial>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_special_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = CBORSpecial::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_special_new_bool(b: bool, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let result = CBORSpecial::new_bool(b);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_special_new_unassigned(u_long: i64, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let u  = u_long as u8;
    let result = CBORSpecial::new_unassigned(u);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_special_new_break(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let result = CBORSpecial::new_break();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_special_new_null(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let result = CBORSpecial::new_null();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_special_new_undefined(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let result = CBORSpecial::new_undefined();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_special_kind(self_rptr: RPtr, result: &mut i32, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORSpecial>()?;
    let result = self_ref.kind();
    Ok::<i32, String>(result as i32)
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_special_as_bool(self_rptr: RPtr, result: &mut bool, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORSpecial>()?;
    let result = self_ref.as_bool();
    Ok::<Option<bool>, String>(result)
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_special_as_float(self_rptr: RPtr, result: &mut f64, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORSpecial>()?;
    let result = self_ref.as_float();
    Ok::<Option<f64>, String>(result.map(|v| v as f64))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_special_as_unassigned(self_rptr: RPtr, result: &mut i64, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORSpecial>()?;
    let result = self_ref.as_unassigned();
    Ok::<Option<i64>, String>(result.map(|v| v as i64))
  })
  .response_nullable(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORValue>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = CBORValue::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_new_int(int_value_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let int_value = int_value_rptr.typed_ref::<Int>()?;
    let result = CBORValue::new_int(int_value);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_new_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = CBORValue::new_bytes(bytes);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_new_text(text_str: CharPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let text : String = text_str.into_str();
    let result = CBORValue::new_text(text);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_new_array(arr_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let arr = arr_rptr.typed_ref::<CBORArray>()?;
    let result = CBORValue::new_array(arr);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_new_object(obj_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let obj = obj_rptr.typed_ref::<CBORObject>()?;
    let result = CBORValue::new_object(obj);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_new_tagged(tagged_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let tagged = tagged_rptr.typed_ref::<TaggedCBOR>()?;
    let result = CBORValue::new_tagged(tagged);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_new_special(special_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let special = special_rptr.typed_ref::<CBORSpecial>()?;
    let result = CBORValue::new_special(special);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_from_label(label_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let label = label_rptr.typed_ref::<Label>()?;
    let result = CBORValue::from_label(label);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_kind(self_rptr: RPtr, result: &mut i32, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORValue>()?;
    let result = self_ref.kind();
    Ok::<i32, String>(result as i32)
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_as_int(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORValue>()?;
    let result = self_ref.as_int();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_as_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORValue>()?;
    let result = self_ref.as_bytes();
    Ok::<Option<DataPtr>, String>(result.into_option())
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_as_text(self_rptr: RPtr, result: &mut CharPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORValue>()?;
    let result = self_ref.as_text();
    Ok::<Option<CharPtr>, String>(result.into_opt_cstr())
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_as_array(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORValue>()?;
    let result = self_ref.as_array();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_as_object(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORValue>()?;
    let result = self_ref.as_object();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_as_tagged(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORValue>()?;
    let result = self_ref.as_tagged();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_b_o_r_value_as_special(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CBORValue>()?;
    let result = self_ref.as_special();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_encrypt_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEEncrypt>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_encrypt_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = COSEEncrypt::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_encrypt_headers(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEEncrypt>()?;
    let result = self_ref.headers();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_encrypt_ciphertext(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEEncrypt>()?;
    let result = self_ref.ciphertext();
    Ok::<Option<DataPtr>, String>(result.into_option())
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_encrypt_recipients(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEEncrypt>()?;
    let result = self_ref.recipients();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_encrypt_new(headers_rptr: RPtr, recipients_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let headers = headers_rptr.typed_ref::<Headers>()?;
    let recipients = recipients_rptr.typed_ref::<COSERecipients>()?;
    let result = COSEEncrypt::new(headers, None, recipients);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_encrypt_new_with_ciphertext(headers_rptr: RPtr, ciphertext_data: *const u8, ciphertext_len: usize, recipients_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let headers = headers_rptr.typed_ref::<Headers>()?;
    let ciphertext = from_raw_parts(ciphertext_data, ciphertext_len).to_vec();
    let recipients = recipients_rptr.typed_ref::<COSERecipients>()?;
    let result = COSEEncrypt::new(headers, Some(ciphertext), recipients);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}




#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_encrypt0_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEEncrypt0>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_encrypt0_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = COSEEncrypt0::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_encrypt0_headers(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEEncrypt0>()?;
    let result = self_ref.headers();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_encrypt0_ciphertext(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEEncrypt0>()?;
    let result = self_ref.ciphertext();
    Ok::<Option<DataPtr>, String>(result.into_option())
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_encrypt0_new(headers_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let headers = headers_rptr.typed_ref::<Headers>()?;
    let result = COSEEncrypt0::new(headers, None);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_encrypt0_new_with_ciphertext(headers_rptr: RPtr, ciphertext_data: *const u8, ciphertext_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let headers = headers_rptr.typed_ref::<Headers>()?;
    let ciphertext = from_raw_parts(ciphertext_data, ciphertext_len).to_vec();
    let result = COSEEncrypt0::new(headers, Some(ciphertext));
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}




#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEKey>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = COSEKey::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_set_key_type(self_rptr: RPtr, key_type_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEKey>()?;
    let key_type = key_type_rptr.typed_ref::<Label>()?;
    self_ref.set_key_type(key_type);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_key_type(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEKey>()?;
    let result = self_ref.key_type();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_set_key_id(self_rptr: RPtr, key_id_data: *const u8, key_id_len: usize, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEKey>()?;
    let key_id = from_raw_parts(key_id_data, key_id_len).to_vec();
    self_ref.set_key_id(key_id);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_key_id(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEKey>()?;
    let result = self_ref.key_id();
    Ok::<Option<DataPtr>, String>(result.into_option())
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_set_algorithm_id(self_rptr: RPtr, algorithm_id_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEKey>()?;
    let algorithm_id = algorithm_id_rptr.typed_ref::<Label>()?;
    self_ref.set_algorithm_id(algorithm_id);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_algorithm_id(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEKey>()?;
    let result = self_ref.algorithm_id();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_set_key_ops(self_rptr: RPtr, key_ops_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEKey>()?;
    let key_ops = key_ops_rptr.typed_ref::<Labels>()?;
    self_ref.set_key_ops(key_ops);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_key_ops(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEKey>()?;
    let result = self_ref.key_ops();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_set_base_init_vector(self_rptr: RPtr, base_init_vector_data: *const u8, base_init_vector_len: usize, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEKey>()?;
    let base_init_vector = from_raw_parts(base_init_vector_data, base_init_vector_len).to_vec();
    self_ref.set_base_init_vector(base_init_vector);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_base_init_vector(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEKey>()?;
    let result = self_ref.base_init_vector();
    Ok::<Option<DataPtr>, String>(result.into_option())
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_header(self_rptr: RPtr, label_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEKey>()?;
    let label = label_rptr.typed_ref::<Label>()?;
    let result = self_ref.header(label);
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_set_header(self_rptr: RPtr, label_rptr: RPtr, value_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSEKey>()?;
    let label = label_rptr.typed_ref::<Label>()?;
    let value = value_rptr.typed_ref::<CBORValue>()?;
    self_ref.set_header(label, value).into_result()?;
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_key_new(key_type_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let key_type = key_type_rptr.typed_ref::<Label>()?;
    let result = COSEKey::new(key_type);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_recipient_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSERecipient>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_recipient_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = COSERecipient::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_recipient_headers(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSERecipient>()?;
    let result = self_ref.headers();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_recipient_ciphertext(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSERecipient>()?;
    let result = self_ref.ciphertext();
    Ok::<Option<DataPtr>, String>(result.into_option())
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_recipient_new(headers_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let headers = headers_rptr.typed_ref::<Headers>()?;
    let result = COSERecipient::new(headers, None);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_recipient_new_with_ciphertext(headers_rptr: RPtr, ciphertext_data: *const u8, ciphertext_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let headers = headers_rptr.typed_ref::<Headers>()?;
    let ciphertext = from_raw_parts(ciphertext_data, ciphertext_len).to_vec();
    let result = COSERecipient::new(headers, Some(ciphertext));
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}




#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_recipients_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSERecipients>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_recipients_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = COSERecipients::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_recipients_new(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let result = COSERecipients::new();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_recipients_len(self_rptr: RPtr, result: &mut i64, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSERecipients>()?;
    let result = self_ref.len();
    Ok::<i64, String>(result as i64)
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_recipients_get(self_rptr: RPtr, index_long: i64, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSERecipients>()?;
    let index  = index_long as usize;
    let result = self_ref.get(index);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_recipients_add(self_rptr: RPtr, elem_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSERecipients>()?;
    let elem = elem_rptr.typed_ref::<COSERecipient>()?;
    self_ref.add(elem);
    Ok(())
  })
  .response(&mut (),  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = COSESign::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign_headers(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign>()?;
    let result = self_ref.headers();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign_payload(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign>()?;
    let result = self_ref.payload();
    Ok::<Option<DataPtr>, String>(result.into_option())
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign_signatures(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign>()?;
    let result = self_ref.signatures();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign_new(headers_rptr: RPtr, signatures_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let headers = headers_rptr.typed_ref::<Headers>()?;
    let signatures = signatures_rptr.typed_ref::<COSESignatures>()?;
    let result = COSESign::new(headers, None, signatures);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign_new_with_payload(headers_rptr: RPtr, payload_data: *const u8, payload_len: usize, signatures_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let headers = headers_rptr.typed_ref::<Headers>()?;
    let payload = from_raw_parts(payload_data, payload_len).to_vec();
    let signatures = signatures_rptr.typed_ref::<COSESignatures>()?;
    let result = COSESign::new(headers, Some(payload), signatures);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}




#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign1>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = COSESign1::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_headers(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign1>()?;
    let result = self_ref.headers();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_payload(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign1>()?;
    let result = self_ref.payload();
    Ok::<Option<DataPtr>, String>(result.into_option())
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_signature(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign1>()?;
    let result = self_ref.signature();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_signed_data(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign1>()?;
    let result = self_ref.signed_data(None, None).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_signed_data_with_external_aad(self_rptr: RPtr, external_aad_data: *const u8, external_aad_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign1>()?;
    let external_aad = from_raw_parts(external_aad_data, external_aad_len).to_vec();
    let result = self_ref.signed_data(Some(external_aad), None).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_signed_data_with_external_payload(self_rptr: RPtr, external_payload_data: *const u8, external_payload_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign1>()?;
    let external_payload = from_raw_parts(external_payload_data, external_payload_len).to_vec();
    let result = self_ref.signed_data(None, Some(external_payload)).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_signed_data_with_external_aad_external_payload(self_rptr: RPtr, external_aad_data: *const u8, external_aad_len: usize, external_payload_data: *const u8, external_payload_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign1>()?;
    let external_aad = from_raw_parts(external_aad_data, external_aad_len).to_vec();
    let external_payload = from_raw_parts(external_payload_data, external_payload_len).to_vec();
    let result = self_ref.signed_data(Some(external_aad), Some(external_payload)).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_new(headers_rptr: RPtr, signature_data: *const u8, signature_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let headers = headers_rptr.typed_ref::<Headers>()?;
    let signature = from_raw_parts(signature_data, signature_len).to_vec();
    let result = COSESign1::new(headers, None, signature);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_new_with_payload(headers_rptr: RPtr, payload_data: *const u8, payload_len: usize, signature_data: *const u8, signature_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let headers = headers_rptr.typed_ref::<Headers>()?;
    let payload = from_raw_parts(payload_data, payload_len).to_vec();
    let signature = from_raw_parts(signature_data, signature_len).to_vec();
    let result = COSESign1::new(headers, Some(payload), signature);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}




#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_builder_new(headers_rptr: RPtr, payload_data: *const u8, payload_len: usize, is_payload_external: bool, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let headers = headers_rptr.typed_ref::<Headers>()?;
    let payload = from_raw_parts(payload_data, payload_len).to_vec();
    let result = COSESign1Builder::new(headers, payload, is_payload_external);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_builder_hash_payload(self_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign1Builder>()?;
    self_ref.hash_payload();
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_builder_set_external_aad(self_rptr: RPtr, external_aad_data: *const u8, external_aad_len: usize, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign1Builder>()?;
    let external_aad = from_raw_parts(external_aad_data, external_aad_len).to_vec();
    self_ref.set_external_aad(external_aad);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_builder_make_data_to_sign(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign1Builder>()?;
    let result = self_ref.make_data_to_sign();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign1_builder_build(self_rptr: RPtr, signed_sig_structure_data: *const u8, signed_sig_structure_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESign1Builder>()?;
    let signed_sig_structure = from_raw_parts(signed_sig_structure_data, signed_sig_structure_len).to_vec();
    let result = self_ref.build(signed_sig_structure);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign_builder_new(headers_rptr: RPtr, payload_data: *const u8, payload_len: usize, is_payload_external: bool, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let headers = headers_rptr.typed_ref::<Headers>()?;
    let payload = from_raw_parts(payload_data, payload_len).to_vec();
    let result = COSESignBuilder::new(headers, payload, is_payload_external);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign_builder_hash_payload(self_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESignBuilder>()?;
    self_ref.hash_payload();
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign_builder_set_external_aad(self_rptr: RPtr, external_aad_data: *const u8, external_aad_len: usize, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESignBuilder>()?;
    let external_aad = from_raw_parts(external_aad_data, external_aad_len).to_vec();
    self_ref.set_external_aad(external_aad);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign_builder_make_data_to_sign(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESignBuilder>()?;
    let result = self_ref.make_data_to_sign();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_sign_builder_build(self_rptr: RPtr, signed_sig_structure_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESignBuilder>()?;
    let signed_sig_structure = signed_sig_structure_rptr.typed_ref::<COSESignatures>()?;
    let result = self_ref.build(signed_sig_structure);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_signature_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESignature>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_signature_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = COSESignature::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_signature_headers(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESignature>()?;
    let result = self_ref.headers();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_signature_signature(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESignature>()?;
    let result = self_ref.signature();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_signature_new(headers_rptr: RPtr, signature_data: *const u8, signature_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let headers = headers_rptr.typed_ref::<Headers>()?;
    let signature = from_raw_parts(signature_data, signature_len).to_vec();
    let result = COSESignature::new(headers, signature);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_signatures_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESignatures>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_signatures_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = COSESignatures::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_signatures_new(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let result = COSESignatures::new();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_signatures_len(self_rptr: RPtr, result: &mut i64, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESignatures>()?;
    let result = self_ref.len();
    Ok::<i64, String>(result as i64)
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_signatures_get(self_rptr: RPtr, index_long: i64, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESignatures>()?;
    let index  = index_long as usize;
    let result = self_ref.get(index);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_c_o_s_e_signatures_add(self_rptr: RPtr, elem_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<COSESignatures>()?;
    let elem = elem_rptr.typed_ref::<COSESignature>()?;
    self_ref.add(elem);
    Ok(())
  })
  .response(&mut (),  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_counter_signature_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CounterSignature>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_counter_signature_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = CounterSignature::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_counter_signature_new_single(cose_signature_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let cose_signature = cose_signature_rptr.typed_ref::<COSESignature>()?;
    let result = CounterSignature::new_single(cose_signature);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_counter_signature_new_multi(cose_signatures_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let cose_signatures = cose_signatures_rptr.typed_ref::<COSESignatures>()?;
    let result = CounterSignature::new_multi(cose_signatures);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_counter_signature_signatures(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<CounterSignature>()?;
    let result = self_ref.signatures();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_ed_d_s_a25519_key_new(pubkey_bytes_data: *const u8, pubkey_bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let pubkey_bytes = from_raw_parts(pubkey_bytes_data, pubkey_bytes_len).to_vec();
    let result = EdDSA25519Key::new(pubkey_bytes);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_ed_d_s_a25519_key_set_private_key(self_rptr: RPtr, private_key_bytes_data: *const u8, private_key_bytes_len: usize, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<EdDSA25519Key>()?;
    let private_key_bytes = from_raw_parts(private_key_bytes_data, private_key_bytes_len).to_vec();
    self_ref.set_private_key(private_key_bytes);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_ed_d_s_a25519_key_is_for_signing(self_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<EdDSA25519Key>()?;
    self_ref.is_for_signing();
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_ed_d_s_a25519_key_is_for_verifying(self_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<EdDSA25519Key>()?;
    self_ref.is_for_verifying();
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_ed_d_s_a25519_key_build(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<EdDSA25519Key>()?;
    let result = self_ref.build();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = HeaderMap::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_set_algorithm_id(self_rptr: RPtr, algorithm_id_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let algorithm_id = algorithm_id_rptr.typed_ref::<Label>()?;
    self_ref.set_algorithm_id(algorithm_id);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_algorithm_id(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let result = self_ref.algorithm_id();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_set_criticality(self_rptr: RPtr, criticality_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let criticality = criticality_rptr.typed_ref::<Labels>()?;
    self_ref.set_criticality(criticality);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_criticality(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let result = self_ref.criticality();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_set_content_type(self_rptr: RPtr, content_type_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let content_type = content_type_rptr.typed_ref::<Label>()?;
    self_ref.set_content_type(content_type);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_content_type(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let result = self_ref.content_type();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_set_key_id(self_rptr: RPtr, key_id_data: *const u8, key_id_len: usize, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let key_id = from_raw_parts(key_id_data, key_id_len).to_vec();
    self_ref.set_key_id(key_id);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_key_id(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let result = self_ref.key_id();
    Ok::<Option<DataPtr>, String>(result.into_option())
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_set_init_vector(self_rptr: RPtr, init_vector_data: *const u8, init_vector_len: usize, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let init_vector = from_raw_parts(init_vector_data, init_vector_len).to_vec();
    self_ref.set_init_vector(init_vector);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_init_vector(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let result = self_ref.init_vector();
    Ok::<Option<DataPtr>, String>(result.into_option())
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_set_partial_init_vector(self_rptr: RPtr, partial_init_vector_data: *const u8, partial_init_vector_len: usize, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let partial_init_vector = from_raw_parts(partial_init_vector_data, partial_init_vector_len).to_vec();
    self_ref.set_partial_init_vector(partial_init_vector);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_partial_init_vector(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let result = self_ref.partial_init_vector();
    Ok::<Option<DataPtr>, String>(result.into_option())
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_set_counter_signature(self_rptr: RPtr, counter_signature_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let counter_signature = counter_signature_rptr.typed_ref::<CounterSignature>()?;
    self_ref.set_counter_signature(counter_signature);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_counter_signature(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let result = self_ref.counter_signature();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_header(self_rptr: RPtr, label_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let label = label_rptr.typed_ref::<Label>()?;
    let result = self_ref.header(label);
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_set_header(self_rptr: RPtr, label_rptr: RPtr, value_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let label = label_rptr.typed_ref::<Label>()?;
    let value = value_rptr.typed_ref::<CBORValue>()?;
    self_ref.set_header(label, value).into_result()?;
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_keys(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<HeaderMap>()?;
    let result = self_ref.keys();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_header_map_new(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let result = HeaderMap::new();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_headers_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Headers>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_headers_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = Headers::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_headers_protected(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Headers>()?;
    let result = self_ref.protected();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_headers_unprotected(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Headers>()?;
    let result = self_ref.unprotected();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_headers_new(protected__rptr: RPtr, unprotected__rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let protected_ = protected__rptr.typed_ref::<ProtectedHeaderMap>()?;
    let unprotected_ = unprotected__rptr.typed_ref::<HeaderMap>()?;
    let result = Headers::new(protected_, unprotected_);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_int_new(x_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let x = x_rptr.typed_ref::<BigNum>()?.clone();
    let result = Int::new(x);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_int_new_negative(x_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let x = x_rptr.typed_ref::<BigNum>()?.clone();
    let result = Int::new_negative(x);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_int_new_i32(x_long: i64, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let x  = x_long as i32;
    let result = Int::new_i32(x);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_int_is_positive(self_rptr: RPtr, result: &mut bool, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Int>()?;
    let result = self_ref.is_positive();
    Ok::<bool, String>(result)
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_int_as_positive(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Int>()?;
    let result = self_ref.as_positive();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_int_as_negative(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Int>()?;
    let result = self_ref.as_negative();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_int_as_i32(self_rptr: RPtr, result: &mut i64, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Int>()?;
    let result = self_ref.as_i32();
    Ok::<Option<i64>, String>(result.map(|v| v as i64))
  })
  .response_nullable(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_label_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Label>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_label_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = Label::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_label_new_int(int_value_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let int_value = int_value_rptr.typed_ref::<Int>()?;
    let result = Label::new_int(int_value);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_label_new_text(text_str: CharPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let text : String = text_str.into_str();
    let result = Label::new_text(text);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_label_kind(self_rptr: RPtr, result: &mut i32, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Label>()?;
    let result = self_ref.kind();
    Ok::<i32, String>(result as i32)
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_label_as_int(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Label>()?;
    let result = self_ref.as_int();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_label_as_text(self_rptr: RPtr, result: &mut CharPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Label>()?;
    let result = self_ref.as_text();
    Ok::<Option<CharPtr>, String>(result.into_opt_cstr())
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_label_from_algorithm_id(id_int: i32, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let id = id_int.to_enum()?;
    let result = Label::from_algorithm_id(id);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_label_from_key_type(key_type_int: i32, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let key_type = key_type_int.to_enum()?;
    let result = Label::from_key_type(key_type);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_label_from_ec_key(ec_key_int: i32, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let ec_key = ec_key_int.to_enum()?;
    let result = Label::from_ec_key(ec_key);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_label_from_curve_type(curve_type_int: i32, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let curve_type = curve_type_int.to_enum()?;
    let result = Label::from_curve_type(curve_type);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_label_from_key_operation(key_op_int: i32, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let key_op = key_op_int.to_enum()?;
    let result = Label::from_key_operation(key_op);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_labels_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Labels>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_labels_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = Labels::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_labels_new(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let result = Labels::new();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_labels_len(self_rptr: RPtr, result: &mut i64, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Labels>()?;
    let result = self_ref.len();
    Ok::<i64, String>(result as i64)
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_labels_get(self_rptr: RPtr, index_long: i64, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Labels>()?;
    let index  = index_long as usize;
    let result = self_ref.get(index);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_labels_add(self_rptr: RPtr, elem_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<Labels>()?;
    let elem = elem_rptr.typed_ref::<Label>()?;
    self_ref.add(elem);
    Ok(())
  })
  .response(&mut (),  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_password_encryption_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<PasswordEncryption>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_password_encryption_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = PasswordEncryption::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_password_encryption_new(data_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let data = data_rptr.typed_ref::<COSEEncrypt0>()?;
    let result = PasswordEncryption::new(data);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_protected_header_map_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<ProtectedHeaderMap>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_protected_header_map_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = ProtectedHeaderMap::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_protected_header_map_new_empty(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let result = ProtectedHeaderMap::new_empty();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_protected_header_map_new(header_map_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let header_map = header_map_rptr.typed_ref::<HeaderMap>()?;
    let result = ProtectedHeaderMap::new(header_map);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_protected_header_map_deserialized_headers(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<ProtectedHeaderMap>()?;
    let result = self_ref.deserialized_headers();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_pub_key_encryption_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<PubKeyEncryption>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_pub_key_encryption_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = PubKeyEncryption::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_pub_key_encryption_new(data_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let data = data_rptr.typed_ref::<COSEEncrypt>()?;
    let result = PubKeyEncryption::new(data);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_sig_structure_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<SigStructure>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_sig_structure_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = SigStructure::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_sig_structure_context(self_rptr: RPtr, result: &mut i32, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<SigStructure>()?;
    let result = self_ref.context();
    Ok::<i32, String>(result as i32)
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_sig_structure_body_protected(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<SigStructure>()?;
    let result = self_ref.body_protected();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_sig_structure_sign_protected(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<SigStructure>()?;
    let result = self_ref.sign_protected();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_sig_structure_external_aad(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<SigStructure>()?;
    let result = self_ref.external_aad();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_sig_structure_payload(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<SigStructure>()?;
    let result = self_ref.payload();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_sig_structure_set_sign_protected(self_rptr: RPtr, sign_protected_rptr: RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<SigStructure>()?;
    let sign_protected = sign_protected_rptr.typed_ref::<ProtectedHeaderMap>()?;
    self_ref.set_sign_protected(sign_protected);
    Ok(())
  })
  .response(&mut (),  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_sig_structure_new(context_int: i32, body_protected_rptr: RPtr, external_aad_data: *const u8, external_aad_len: usize, payload_data: *const u8, payload_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let context = context_int.to_enum()?;
    let body_protected = body_protected_rptr.typed_ref::<ProtectedHeaderMap>()?;
    let external_aad = from_raw_parts(external_aad_data, external_aad_len).to_vec();
    let payload = from_raw_parts(payload_data, payload_len).to_vec();
    let result = SigStructure::new(context, body_protected, external_aad, payload);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_signed_message_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<SignedMessage>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_signed_message_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = SignedMessage::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_signed_message_new_cose_sign(cose_sign_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let cose_sign = cose_sign_rptr.typed_ref::<COSESign>()?;
    let result = SignedMessage::new_cose_sign(cose_sign);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_signed_message_new_cose_sign1(cose_sign1_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let cose_sign1 = cose_sign1_rptr.typed_ref::<COSESign1>()?;
    let result = SignedMessage::new_cose_sign1(cose_sign1);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_signed_message_from_user_facing_encoding(s_str: CharPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let s: &str = s_str.into_str();
    let result = SignedMessage::from_user_facing_encoding(s).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_signed_message_to_user_facing_encoding(self_rptr: RPtr, result: &mut CharPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<SignedMessage>()?;
    let result = self_ref.to_user_facing_encoding();
    Ok::<CharPtr, String>(result.into_cstr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_signed_message_kind(self_rptr: RPtr, result: &mut i32, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<SignedMessage>()?;
    let result = self_ref.kind();
    Ok::<i32, String>(result as i32)
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_signed_message_as_cose_sign(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<SignedMessage>()?;
    let result = self_ref.as_cose_sign();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_signed_message_as_cose_sign1(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<SignedMessage>()?;
    let result = self_ref.as_cose_sign1();
    Ok::<Option<RPtr>, String>(result.map(|v| v.rptr()))
  })
  .response_nullable(result,  error)
}



#[no_mangle]
pub unsafe extern "C" fn msl_bridge_tagged_c_b_o_r_to_bytes(self_rptr: RPtr, result: &mut DataPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<TaggedCBOR>()?;
    let result = self_ref.to_bytes();
    Ok::<DataPtr, String>(result.into())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_tagged_c_b_o_r_from_bytes(bytes_data: *const u8, bytes_len: usize, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let bytes = from_raw_parts(bytes_data, bytes_len).to_vec();
    let result = TaggedCBOR::from_bytes(bytes).into_result()?;
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_tagged_c_b_o_r_tag(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<TaggedCBOR>()?;
    let result = self_ref.tag();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_tagged_c_b_o_r_value(self_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let self_ref = self_rptr.typed_ref::<TaggedCBOR>()?;
    let result = self_ref.value();
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}


#[no_mangle]
pub unsafe extern "C" fn msl_bridge_tagged_c_b_o_r_new(tag_rptr: RPtr, value_rptr: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| { 
    let tag = tag_rptr.typed_ref::<BigNum>()?.clone();
    let value = value_rptr.typed_ref::<CBORValue>()?;
    let result = TaggedCBOR::new(tag, value);
    Ok::<RPtr, String>(result.rptr())
  })
  .response(result,  error)
}



