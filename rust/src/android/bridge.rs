use super::bridge_tools::ptr_j::*;
use super::bridge_tools::result::*;
use crate::panic::{handle_exception_result, Zip, ToResult};
use crate::ptr::RPtrRepresentable;
use crate::ptr_impl::*;
use crate::enum_maps::*;
use crate::arrays::*;
use super::bridge_tools::boxing::*;
use super::bridge_tools::unboxing::*;
use super::bridge_tools::primitives::*;
use super::bridge_tools::utils::*;
use super::bridge_tools::string::*;
use jni::objects::{JObject, JString};
use jni::sys::{jlong, jint, jobject, jboolean, jbyteArray};
use jni::JNIEnv;
use std::convert::TryFrom;
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


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1bigNumToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<BigNum>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1bigNumFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = BigNum::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1bigNumFromStr(env: JNIEnv, _: JObject, string_str: JString) -> jobject {
  handle_exception_result(|| { 
    let string = string_str.string(&env)?;
    let result = BigNum::from_str(&string).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1bigNumToStr(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<BigNum>()?;
    let result = self_rptr.to_str();
    result.jstring(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1bigNumCheckedMul(env: JNIEnv, _: JObject, self_ptr: JRPtr, other_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<BigNum>()?;
    let other_jrptr = other_ptr.rptr(&env)?;
    let other = other_jrptr.typed_ref::<BigNum>()?;
    let result = self_rptr.checked_mul(other).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1bigNumCheckedAdd(env: JNIEnv, _: JObject, self_ptr: JRPtr, other_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<BigNum>()?;
    let other_jrptr = other_ptr.rptr(&env)?;
    let other = other_jrptr.typed_ref::<BigNum>()?;
    let result = self_rptr.checked_add(other).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1bigNumCheckedSub(env: JNIEnv, _: JObject, self_ptr: JRPtr, other_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<BigNum>()?;
    let other_jrptr = other_ptr.rptr(&env)?;
    let other = other_jrptr.typed_ref::<BigNum>()?;
    let result = self_rptr.checked_sub(other).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORArrayToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORArray>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORArrayFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = CBORArray::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORArrayNew(env: JNIEnv, _: JObject) -> jobject {
  handle_exception_result(|| { 
    let result = CBORArray::new();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORArrayLen(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORArray>()?;
    let result = self_rptr.len();
    result.into_jlong().jobject(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORArrayGet(env: JNIEnv, _: JObject, self_ptr: JRPtr, index_jlong: jlong) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORArray>()?;
    let index = usize::try_from_jlong(index_jlong)?;
    let result = self_rptr.get(index);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORArrayAdd(env: JNIEnv, _: JObject, self_ptr: JRPtr, elem_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORArray>()?;
    let elem_jrptr = elem_ptr.rptr(&env)?;
    let elem = elem_jrptr.typed_ref::<CBORValue>()?;
    self_rptr.add(elem);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORArraySetDefiniteEncoding(env: JNIEnv, _: JObject, self_ptr: JRPtr, use_definite_jboolean: jboolean) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORArray>()?;
    let use_definite = use_definite_jboolean.into_bool();
    self_rptr.set_definite_encoding(use_definite);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORArrayIsDefinite(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORArray>()?;
    let result = self_rptr.is_definite();
    result.into_jboolean().jobject(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORObjectToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORObject>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORObjectFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = CBORObject::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORObjectNew(env: JNIEnv, _: JObject) -> jobject {
  handle_exception_result(|| { 
    let result = CBORObject::new();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORObjectLen(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORObject>()?;
    let result = self_rptr.len();
    result.into_jlong().jobject(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORObjectInsert(env: JNIEnv, _: JObject, self_ptr: JRPtr, key_ptr: JRPtr, value_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORObject>()?;
    let key_jrptr = key_ptr.rptr(&env)?;
    let key = key_jrptr.typed_ref::<CBORValue>()?;
    let value_jrptr = value_ptr.rptr(&env)?;
    let value = value_jrptr.typed_ref::<CBORValue>()?;
    let result = self_rptr.insert(key, value);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORObjectGet(env: JNIEnv, _: JObject, self_ptr: JRPtr, key_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORObject>()?;
    let key_jrptr = key_ptr.rptr(&env)?;
    let key = key_jrptr.typed_ref::<CBORValue>()?;
    let result = self_rptr.get(key);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORObjectKeys(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORObject>()?;
    let result = self_rptr.keys();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORObjectSetDefiniteEncoding(env: JNIEnv, _: JObject, self_ptr: JRPtr, use_definite_jboolean: jboolean) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORObject>()?;
    let use_definite = use_definite_jboolean.into_bool();
    self_rptr.set_definite_encoding(use_definite);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORObjectIsDefinite(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORObject>()?;
    let result = self_rptr.is_definite();
    result.into_jboolean().jobject(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORSpecialToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORSpecial>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORSpecialFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = CBORSpecial::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORSpecialNewBool(env: JNIEnv, _: JObject, b_jboolean: jboolean) -> jobject {
  handle_exception_result(|| { 
    let b = b_jboolean.into_bool();
    let result = CBORSpecial::new_bool(b);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORSpecialNewUnassigned(env: JNIEnv, _: JObject, u_jlong: jlong) -> jobject {
  handle_exception_result(|| { 
    let u = u8::try_from_jlong(u_jlong)?;
    let result = CBORSpecial::new_unassigned(u);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORSpecialNewBreak(env: JNIEnv, _: JObject) -> jobject {
  handle_exception_result(|| { 
    let result = CBORSpecial::new_break();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORSpecialNewNull(env: JNIEnv, _: JObject) -> jobject {
  handle_exception_result(|| { 
    let result = CBORSpecial::new_null();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORSpecialNewUndefined(env: JNIEnv, _: JObject) -> jobject {
  handle_exception_result(|| { 
    let result = CBORSpecial::new_undefined();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORSpecialKind(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORSpecial>()?;
    let result = self_rptr.kind();
    (result.to_i32() as jint).jobject(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORSpecialAsBool(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORSpecial>()?;
    let result = self_rptr.as_bool();
    result.into_jboolean().jobject(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORSpecialAsFloat(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORSpecial>()?;
    let result = self_rptr.as_float();
    result.into_jdouble().jobject(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORSpecialAsUnassigned(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORSpecial>()?;
    let result = self_rptr.as_unassigned();
    result.into_jlong().jobject(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORValue>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = CBORValue::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueNewInt(env: JNIEnv, _: JObject, int_value_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let int_value_jrptr = int_value_ptr.rptr(&env)?;
    let int_value = int_value_jrptr.typed_ref::<Int>()?;
    let result = CBORValue::new_int(int_value);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueNewBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = CBORValue::new_bytes(bytes);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueNewText(env: JNIEnv, _: JObject, text_str: JString) -> jobject {
  handle_exception_result(|| { 
    let text = text_str.string(&env)?;
    let result = CBORValue::new_text(text);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueNewArray(env: JNIEnv, _: JObject, arr_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let arr_jrptr = arr_ptr.rptr(&env)?;
    let arr = arr_jrptr.typed_ref::<CBORArray>()?;
    let result = CBORValue::new_array(arr);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueNewObject(env: JNIEnv, _: JObject, obj_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let obj_jrptr = obj_ptr.rptr(&env)?;
    let obj = obj_jrptr.typed_ref::<CBORObject>()?;
    let result = CBORValue::new_object(obj);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueNewTagged(env: JNIEnv, _: JObject, tagged_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let tagged_jrptr = tagged_ptr.rptr(&env)?;
    let tagged = tagged_jrptr.typed_ref::<TaggedCBOR>()?;
    let result = CBORValue::new_tagged(tagged);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueNewSpecial(env: JNIEnv, _: JObject, special_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let special_jrptr = special_ptr.rptr(&env)?;
    let special = special_jrptr.typed_ref::<CBORSpecial>()?;
    let result = CBORValue::new_special(special);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueFromLabel(env: JNIEnv, _: JObject, label_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let label_jrptr = label_ptr.rptr(&env)?;
    let label = label_jrptr.typed_ref::<Label>()?;
    let result = CBORValue::from_label(label);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueKind(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORValue>()?;
    let result = self_rptr.kind();
    (result.to_i32() as jint).jobject(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueAsInt(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORValue>()?;
    let result = self_rptr.as_int();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueAsBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORValue>()?;
    let result = self_rptr.as_bytes();
    match result {
        Some(result) => Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?)),
        None => Ok(JObject::null()),
    }
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueAsText(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORValue>()?;
    let result = self_rptr.as_text();
    result.jstring(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueAsArray(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORValue>()?;
    let result = self_rptr.as_array();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueAsObject(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORValue>()?;
    let result = self_rptr.as_object();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueAsTagged(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORValue>()?;
    let result = self_rptr.as_tagged();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cBORValueAsSpecial(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CBORValue>()?;
    let result = self_rptr.as_special();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEEncryptToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEEncrypt>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEEncryptFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = COSEEncrypt::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEEncryptHeaders(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEEncrypt>()?;
    let result = self_rptr.headers();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEEncryptCiphertext(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEEncrypt>()?;
    let result = self_rptr.ciphertext();
    match result {
        Some(result) => Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?)),
        None => Ok(JObject::null()),
    }
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEEncryptRecipients(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEEncrypt>()?;
    let result = self_rptr.recipients();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEEncryptNew(env: JNIEnv, _: JObject, headers_ptr: JRPtr, recipients_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let headers_jrptr = headers_ptr.rptr(&env)?;
    let headers = headers_jrptr.typed_ref::<Headers>()?;
    let recipients_jrptr = recipients_ptr.rptr(&env)?;
    let recipients = recipients_jrptr.typed_ref::<COSERecipients>()?;
    let result = COSEEncrypt::new(headers, None, recipients);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEEncryptNewWithCiphertext(env: JNIEnv, _: JObject, headers_ptr: JRPtr, ciphertext_jarray: jbyteArray, recipients_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let headers_jrptr = headers_ptr.rptr(&env)?;
    let headers = headers_jrptr.typed_ref::<Headers>()?;
    let ciphertext = env.convert_byte_array(ciphertext_jarray).into_result()?;
    let recipients_jrptr = recipients_ptr.rptr(&env)?;
    let recipients = recipients_jrptr.typed_ref::<COSERecipients>()?;
    let result = COSEEncrypt::new(headers, Some(ciphertext), recipients);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}




#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEEncrypt0ToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEEncrypt0>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEEncrypt0FromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = COSEEncrypt0::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEEncrypt0Headers(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEEncrypt0>()?;
    let result = self_rptr.headers();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEEncrypt0Ciphertext(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEEncrypt0>()?;
    let result = self_rptr.ciphertext();
    match result {
        Some(result) => Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?)),
        None => Ok(JObject::null()),
    }
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEEncrypt0New(env: JNIEnv, _: JObject, headers_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let headers_jrptr = headers_ptr.rptr(&env)?;
    let headers = headers_jrptr.typed_ref::<Headers>()?;
    let result = COSEEncrypt0::new(headers, None);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEEncrypt0NewWithCiphertext(env: JNIEnv, _: JObject, headers_ptr: JRPtr, ciphertext_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let headers_jrptr = headers_ptr.rptr(&env)?;
    let headers = headers_jrptr.typed_ref::<Headers>()?;
    let ciphertext = env.convert_byte_array(ciphertext_jarray).into_result()?;
    let result = COSEEncrypt0::new(headers, Some(ciphertext));
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}




#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeyToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEKey>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeyFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = COSEKey::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeySetKeyType(env: JNIEnv, _: JObject, self_ptr: JRPtr, key_type_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEKey>()?;
    let key_type_jrptr = key_type_ptr.rptr(&env)?;
    let key_type = key_type_jrptr.typed_ref::<Label>()?;
    self_rptr.set_key_type(key_type);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeyKeyType(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEKey>()?;
    let result = self_rptr.key_type();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeySetKeyId(env: JNIEnv, _: JObject, self_ptr: JRPtr, key_id_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEKey>()?;
    let key_id = env.convert_byte_array(key_id_jarray).into_result()?;
    self_rptr.set_key_id(key_id);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeyKeyId(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEKey>()?;
    let result = self_rptr.key_id();
    match result {
        Some(result) => Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?)),
        None => Ok(JObject::null()),
    }
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeySetAlgorithmId(env: JNIEnv, _: JObject, self_ptr: JRPtr, algorithm_id_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEKey>()?;
    let algorithm_id_jrptr = algorithm_id_ptr.rptr(&env)?;
    let algorithm_id = algorithm_id_jrptr.typed_ref::<Label>()?;
    self_rptr.set_algorithm_id(algorithm_id);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeyAlgorithmId(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEKey>()?;
    let result = self_rptr.algorithm_id();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeySetKeyOps(env: JNIEnv, _: JObject, self_ptr: JRPtr, key_ops_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEKey>()?;
    let key_ops_jrptr = key_ops_ptr.rptr(&env)?;
    let key_ops = key_ops_jrptr.typed_ref::<Labels>()?;
    self_rptr.set_key_ops(key_ops);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeyKeyOps(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEKey>()?;
    let result = self_rptr.key_ops();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeySetBaseInitVector(env: JNIEnv, _: JObject, self_ptr: JRPtr, base_init_vector_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEKey>()?;
    let base_init_vector = env.convert_byte_array(base_init_vector_jarray).into_result()?;
    self_rptr.set_base_init_vector(base_init_vector);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeyBaseInitVector(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEKey>()?;
    let result = self_rptr.base_init_vector();
    match result {
        Some(result) => Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?)),
        None => Ok(JObject::null()),
    }
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeyHeader(env: JNIEnv, _: JObject, self_ptr: JRPtr, label_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEKey>()?;
    let label_jrptr = label_ptr.rptr(&env)?;
    let label = label_jrptr.typed_ref::<Label>()?;
    let result = self_rptr.header(label);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeySetHeader(env: JNIEnv, _: JObject, self_ptr: JRPtr, label_ptr: JRPtr, value_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSEKey>()?;
    let label_jrptr = label_ptr.rptr(&env)?;
    let label = label_jrptr.typed_ref::<Label>()?;
    let value_jrptr = value_ptr.rptr(&env)?;
    let value = value_jrptr.typed_ref::<CBORValue>()?;
    self_rptr.set_header(label, value).into_result()?;
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSEKeyNew(env: JNIEnv, _: JObject, key_type_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let key_type_jrptr = key_type_ptr.rptr(&env)?;
    let key_type = key_type_jrptr.typed_ref::<Label>()?;
    let result = COSEKey::new(key_type);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSERecipientToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSERecipient>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSERecipientFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = COSERecipient::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSERecipientHeaders(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSERecipient>()?;
    let result = self_rptr.headers();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSERecipientCiphertext(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSERecipient>()?;
    let result = self_rptr.ciphertext();
    match result {
        Some(result) => Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?)),
        None => Ok(JObject::null()),
    }
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSERecipientNew(env: JNIEnv, _: JObject, headers_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let headers_jrptr = headers_ptr.rptr(&env)?;
    let headers = headers_jrptr.typed_ref::<Headers>()?;
    let result = COSERecipient::new(headers, None);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSERecipientNewWithCiphertext(env: JNIEnv, _: JObject, headers_ptr: JRPtr, ciphertext_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let headers_jrptr = headers_ptr.rptr(&env)?;
    let headers = headers_jrptr.typed_ref::<Headers>()?;
    let ciphertext = env.convert_byte_array(ciphertext_jarray).into_result()?;
    let result = COSERecipient::new(headers, Some(ciphertext));
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}




#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSERecipientsToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSERecipients>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSERecipientsFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = COSERecipients::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSERecipientsNew(env: JNIEnv, _: JObject) -> jobject {
  handle_exception_result(|| { 
    let result = COSERecipients::new();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSERecipientsLen(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSERecipients>()?;
    let result = self_rptr.len();
    result.into_jlong().jobject(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSERecipientsGet(env: JNIEnv, _: JObject, self_ptr: JRPtr, index_jlong: jlong) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSERecipients>()?;
    let index = usize::try_from_jlong(index_jlong)?;
    let result = self_rptr.get(index);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSERecipientsAdd(env: JNIEnv, _: JObject, self_ptr: JRPtr, elem_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSERecipients>()?;
    let elem_jrptr = elem_ptr.rptr(&env)?;
    let elem = elem_jrptr.typed_ref::<COSERecipient>()?;
    self_rptr.add(elem);
    Ok(JObject::null())
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = COSESign::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignHeaders(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign>()?;
    let result = self_rptr.headers();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignPayload(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign>()?;
    let result = self_rptr.payload();
    match result {
        Some(result) => Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?)),
        None => Ok(JObject::null()),
    }
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignSignatures(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign>()?;
    let result = self_rptr.signatures();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignNew(env: JNIEnv, _: JObject, headers_ptr: JRPtr, signatures_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let headers_jrptr = headers_ptr.rptr(&env)?;
    let headers = headers_jrptr.typed_ref::<Headers>()?;
    let signatures_jrptr = signatures_ptr.rptr(&env)?;
    let signatures = signatures_jrptr.typed_ref::<COSESignatures>()?;
    let result = COSESign::new(headers, None, signatures);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignNewWithPayload(env: JNIEnv, _: JObject, headers_ptr: JRPtr, payload_jarray: jbyteArray, signatures_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let headers_jrptr = headers_ptr.rptr(&env)?;
    let headers = headers_jrptr.typed_ref::<Headers>()?;
    let payload = env.convert_byte_array(payload_jarray).into_result()?;
    let signatures_jrptr = signatures_ptr.rptr(&env)?;
    let signatures = signatures_jrptr.typed_ref::<COSESignatures>()?;
    let result = COSESign::new(headers, Some(payload), signatures);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}




#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1ToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign1>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1FromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = COSESign1::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1Headers(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign1>()?;
    let result = self_rptr.headers();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1Payload(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign1>()?;
    let result = self_rptr.payload();
    match result {
        Some(result) => Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?)),
        None => Ok(JObject::null()),
    }
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1Signature(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign1>()?;
    let result = self_rptr.signature();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1SignedData(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign1>()?;
    let result = self_rptr.signed_data(None, None).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1SignedDataWithExternalAad(env: JNIEnv, _: JObject, self_ptr: JRPtr, external_aad_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign1>()?;
    let external_aad = env.convert_byte_array(external_aad_jarray).into_result()?;
    let result = self_rptr.signed_data(Some(external_aad), None).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1SignedDataWithExternalPayload(env: JNIEnv, _: JObject, self_ptr: JRPtr, external_payload_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign1>()?;
    let external_payload = env.convert_byte_array(external_payload_jarray).into_result()?;
    let result = self_rptr.signed_data(None, Some(external_payload)).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1SignedDataWithExternalAadExternalPayload(env: JNIEnv, _: JObject, self_ptr: JRPtr, external_aad_jarray: jbyteArray, external_payload_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign1>()?;
    let external_aad = env.convert_byte_array(external_aad_jarray).into_result()?;
    let external_payload = env.convert_byte_array(external_payload_jarray).into_result()?;
    let result = self_rptr.signed_data(Some(external_aad), Some(external_payload)).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1New(env: JNIEnv, _: JObject, headers_ptr: JRPtr, signature_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let headers_jrptr = headers_ptr.rptr(&env)?;
    let headers = headers_jrptr.typed_ref::<Headers>()?;
    let signature = env.convert_byte_array(signature_jarray).into_result()?;
    let result = COSESign1::new(headers, None, signature);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1NewWithPayload(env: JNIEnv, _: JObject, headers_ptr: JRPtr, payload_jarray: jbyteArray, signature_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let headers_jrptr = headers_ptr.rptr(&env)?;
    let headers = headers_jrptr.typed_ref::<Headers>()?;
    let payload = env.convert_byte_array(payload_jarray).into_result()?;
    let signature = env.convert_byte_array(signature_jarray).into_result()?;
    let result = COSESign1::new(headers, Some(payload), signature);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}




#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1BuilderNew(env: JNIEnv, _: JObject, headers_ptr: JRPtr, payload_jarray: jbyteArray, is_payload_external_jboolean: jboolean) -> jobject {
  handle_exception_result(|| { 
    let headers_jrptr = headers_ptr.rptr(&env)?;
    let headers = headers_jrptr.typed_ref::<Headers>()?;
    let payload = env.convert_byte_array(payload_jarray).into_result()?;
    let is_payload_external = is_payload_external_jboolean.into_bool();
    let result = COSESign1Builder::new(headers, payload, is_payload_external);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1BuilderHashPayload(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign1Builder>()?;
    self_rptr.hash_payload();
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1BuilderSetExternalAad(env: JNIEnv, _: JObject, self_ptr: JRPtr, external_aad_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign1Builder>()?;
    let external_aad = env.convert_byte_array(external_aad_jarray).into_result()?;
    self_rptr.set_external_aad(external_aad);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1BuilderMakeDataToSign(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign1Builder>()?;
    let result = self_rptr.make_data_to_sign();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESign1BuilderBuild(env: JNIEnv, _: JObject, self_ptr: JRPtr, signed_sig_structure_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESign1Builder>()?;
    let signed_sig_structure = env.convert_byte_array(signed_sig_structure_jarray).into_result()?;
    let result = self_rptr.build(signed_sig_structure);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignBuilderNew(env: JNIEnv, _: JObject, headers_ptr: JRPtr, payload_jarray: jbyteArray, is_payload_external_jboolean: jboolean) -> jobject {
  handle_exception_result(|| { 
    let headers_jrptr = headers_ptr.rptr(&env)?;
    let headers = headers_jrptr.typed_ref::<Headers>()?;
    let payload = env.convert_byte_array(payload_jarray).into_result()?;
    let is_payload_external = is_payload_external_jboolean.into_bool();
    let result = COSESignBuilder::new(headers, payload, is_payload_external);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignBuilderHashPayload(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESignBuilder>()?;
    self_rptr.hash_payload();
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignBuilderSetExternalAad(env: JNIEnv, _: JObject, self_ptr: JRPtr, external_aad_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESignBuilder>()?;
    let external_aad = env.convert_byte_array(external_aad_jarray).into_result()?;
    self_rptr.set_external_aad(external_aad);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignBuilderMakeDataToSign(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESignBuilder>()?;
    let result = self_rptr.make_data_to_sign();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignBuilderBuild(env: JNIEnv, _: JObject, self_ptr: JRPtr, signed_sig_structure_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESignBuilder>()?;
    let signed_sig_structure_jrptr = signed_sig_structure_ptr.rptr(&env)?;
    let signed_sig_structure = signed_sig_structure_jrptr.typed_ref::<COSESignatures>()?;
    let result = self_rptr.build(signed_sig_structure);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignatureToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESignature>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignatureFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = COSESignature::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignatureHeaders(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESignature>()?;
    let result = self_rptr.headers();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignatureSignature(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESignature>()?;
    let result = self_rptr.signature();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignatureNew(env: JNIEnv, _: JObject, headers_ptr: JRPtr, signature_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let headers_jrptr = headers_ptr.rptr(&env)?;
    let headers = headers_jrptr.typed_ref::<Headers>()?;
    let signature = env.convert_byte_array(signature_jarray).into_result()?;
    let result = COSESignature::new(headers, signature);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignaturesToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESignatures>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignaturesFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = COSESignatures::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignaturesNew(env: JNIEnv, _: JObject) -> jobject {
  handle_exception_result(|| { 
    let result = COSESignatures::new();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignaturesLen(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESignatures>()?;
    let result = self_rptr.len();
    result.into_jlong().jobject(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignaturesGet(env: JNIEnv, _: JObject, self_ptr: JRPtr, index_jlong: jlong) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESignatures>()?;
    let index = usize::try_from_jlong(index_jlong)?;
    let result = self_rptr.get(index);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1cOSESignaturesAdd(env: JNIEnv, _: JObject, self_ptr: JRPtr, elem_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<COSESignatures>()?;
    let elem_jrptr = elem_ptr.rptr(&env)?;
    let elem = elem_jrptr.typed_ref::<COSESignature>()?;
    self_rptr.add(elem);
    Ok(JObject::null())
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1counterSignatureToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CounterSignature>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1counterSignatureFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = CounterSignature::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1counterSignatureNewSingle(env: JNIEnv, _: JObject, cose_signature_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let cose_signature_jrptr = cose_signature_ptr.rptr(&env)?;
    let cose_signature = cose_signature_jrptr.typed_ref::<COSESignature>()?;
    let result = CounterSignature::new_single(cose_signature);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1counterSignatureNewMulti(env: JNIEnv, _: JObject, cose_signatures_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let cose_signatures_jrptr = cose_signatures_ptr.rptr(&env)?;
    let cose_signatures = cose_signatures_jrptr.typed_ref::<COSESignatures>()?;
    let result = CounterSignature::new_multi(cose_signatures);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1counterSignatureSignatures(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<CounterSignature>()?;
    let result = self_rptr.signatures();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1edDSA25519KeyNew(env: JNIEnv, _: JObject, pubkey_bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let pubkey_bytes = env.convert_byte_array(pubkey_bytes_jarray).into_result()?;
    let result = EdDSA25519Key::new(pubkey_bytes);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1edDSA25519KeySetPrivateKey(env: JNIEnv, _: JObject, self_ptr: JRPtr, private_key_bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<EdDSA25519Key>()?;
    let private_key_bytes = env.convert_byte_array(private_key_bytes_jarray).into_result()?;
    self_rptr.set_private_key(private_key_bytes);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1edDSA25519KeyIsForSigning(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<EdDSA25519Key>()?;
    self_rptr.is_for_signing();
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1edDSA25519KeyIsForVerifying(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<EdDSA25519Key>()?;
    self_rptr.is_for_verifying();
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1edDSA25519KeyBuild(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<EdDSA25519Key>()?;
    let result = self_rptr.build();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = HeaderMap::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapSetAlgorithmId(env: JNIEnv, _: JObject, self_ptr: JRPtr, algorithm_id_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let algorithm_id_jrptr = algorithm_id_ptr.rptr(&env)?;
    let algorithm_id = algorithm_id_jrptr.typed_ref::<Label>()?;
    self_rptr.set_algorithm_id(algorithm_id);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapAlgorithmId(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let result = self_rptr.algorithm_id();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapSetCriticality(env: JNIEnv, _: JObject, self_ptr: JRPtr, criticality_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let criticality_jrptr = criticality_ptr.rptr(&env)?;
    let criticality = criticality_jrptr.typed_ref::<Labels>()?;
    self_rptr.set_criticality(criticality);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapCriticality(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let result = self_rptr.criticality();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapSetContentType(env: JNIEnv, _: JObject, self_ptr: JRPtr, content_type_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let content_type_jrptr = content_type_ptr.rptr(&env)?;
    let content_type = content_type_jrptr.typed_ref::<Label>()?;
    self_rptr.set_content_type(content_type);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapContentType(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let result = self_rptr.content_type();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapSetKeyId(env: JNIEnv, _: JObject, self_ptr: JRPtr, key_id_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let key_id = env.convert_byte_array(key_id_jarray).into_result()?;
    self_rptr.set_key_id(key_id);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapKeyId(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let result = self_rptr.key_id();
    match result {
        Some(result) => Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?)),
        None => Ok(JObject::null()),
    }
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapSetInitVector(env: JNIEnv, _: JObject, self_ptr: JRPtr, init_vector_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let init_vector = env.convert_byte_array(init_vector_jarray).into_result()?;
    self_rptr.set_init_vector(init_vector);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapInitVector(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let result = self_rptr.init_vector();
    match result {
        Some(result) => Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?)),
        None => Ok(JObject::null()),
    }
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapSetPartialInitVector(env: JNIEnv, _: JObject, self_ptr: JRPtr, partial_init_vector_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let partial_init_vector = env.convert_byte_array(partial_init_vector_jarray).into_result()?;
    self_rptr.set_partial_init_vector(partial_init_vector);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapPartialInitVector(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let result = self_rptr.partial_init_vector();
    match result {
        Some(result) => Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?)),
        None => Ok(JObject::null()),
    }
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapSetCounterSignature(env: JNIEnv, _: JObject, self_ptr: JRPtr, counter_signature_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let counter_signature_jrptr = counter_signature_ptr.rptr(&env)?;
    let counter_signature = counter_signature_jrptr.typed_ref::<CounterSignature>()?;
    self_rptr.set_counter_signature(counter_signature);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapCounterSignature(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let result = self_rptr.counter_signature();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapHeader(env: JNIEnv, _: JObject, self_ptr: JRPtr, label_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let label_jrptr = label_ptr.rptr(&env)?;
    let label = label_jrptr.typed_ref::<Label>()?;
    let result = self_rptr.header(label);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapSetHeader(env: JNIEnv, _: JObject, self_ptr: JRPtr, label_ptr: JRPtr, value_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let label_jrptr = label_ptr.rptr(&env)?;
    let label = label_jrptr.typed_ref::<Label>()?;
    let value_jrptr = value_ptr.rptr(&env)?;
    let value = value_jrptr.typed_ref::<CBORValue>()?;
    self_rptr.set_header(label, value).into_result()?;
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapKeys(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<HeaderMap>()?;
    let result = self_rptr.keys();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headerMapNew(env: JNIEnv, _: JObject) -> jobject {
  handle_exception_result(|| { 
    let result = HeaderMap::new();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headersToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Headers>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headersFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = Headers::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headersProtected(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Headers>()?;
    let result = self_rptr.protected();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headersUnprotected(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Headers>()?;
    let result = self_rptr.unprotected();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1headersNew(env: JNIEnv, _: JObject, protected__ptr: JRPtr, unprotected__ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let protected__jrptr = protected__ptr.rptr(&env)?;
    let protected_ = protected__jrptr.typed_ref::<ProtectedHeaderMap>()?;
    let unprotected__jrptr = unprotected__ptr.rptr(&env)?;
    let unprotected_ = unprotected__jrptr.typed_ref::<HeaderMap>()?;
    let result = Headers::new(protected_, unprotected_);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1intNew(env: JNIEnv, _: JObject, x_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let x = x_ptr.rptr(&env)?.typed_ref::<BigNum>()?.clone();
    let result = Int::new(x);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1intNewNegative(env: JNIEnv, _: JObject, x_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let x = x_ptr.rptr(&env)?.typed_ref::<BigNum>()?.clone();
    let result = Int::new_negative(x);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1intNewI32(env: JNIEnv, _: JObject, x_jlong: jlong) -> jobject {
  handle_exception_result(|| { 
    let x = i32::try_from_jlong(x_jlong)?;
    let result = Int::new_i32(x);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1intIsPositive(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Int>()?;
    let result = self_rptr.is_positive();
    result.into_jboolean().jobject(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1intAsPositive(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Int>()?;
    let result = self_rptr.as_positive();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1intAsNegative(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Int>()?;
    let result = self_rptr.as_negative();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1intAsI32(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Int>()?;
    let result = self_rptr.as_i32();
    result.into_jlong().jobject(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Label>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = Label::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelNewInt(env: JNIEnv, _: JObject, int_value_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let int_value_jrptr = int_value_ptr.rptr(&env)?;
    let int_value = int_value_jrptr.typed_ref::<Int>()?;
    let result = Label::new_int(int_value);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelNewText(env: JNIEnv, _: JObject, text_str: JString) -> jobject {
  handle_exception_result(|| { 
    let text = text_str.string(&env)?;
    let result = Label::new_text(text);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelKind(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Label>()?;
    let result = self_rptr.kind();
    (result.to_i32() as jint).jobject(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelAsInt(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Label>()?;
    let result = self_rptr.as_int();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelAsText(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Label>()?;
    let result = self_rptr.as_text();
    result.jstring(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelFromAlgorithmId(env: JNIEnv, _: JObject, id_jint: jint) -> jobject {
  handle_exception_result(|| { 
    let id = id_jint.to_enum()?;
    let result = Label::from_algorithm_id(id);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelFromKeyType(env: JNIEnv, _: JObject, key_type_jint: jint) -> jobject {
  handle_exception_result(|| { 
    let key_type = key_type_jint.to_enum()?;
    let result = Label::from_key_type(key_type);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelFromEcKey(env: JNIEnv, _: JObject, ec_key_jint: jint) -> jobject {
  handle_exception_result(|| { 
    let ec_key = ec_key_jint.to_enum()?;
    let result = Label::from_ec_key(ec_key);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelFromCurveType(env: JNIEnv, _: JObject, curve_type_jint: jint) -> jobject {
  handle_exception_result(|| { 
    let curve_type = curve_type_jint.to_enum()?;
    let result = Label::from_curve_type(curve_type);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelFromKeyOperation(env: JNIEnv, _: JObject, key_op_jint: jint) -> jobject {
  handle_exception_result(|| { 
    let key_op = key_op_jint.to_enum()?;
    let result = Label::from_key_operation(key_op);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelsToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Labels>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelsFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = Labels::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelsNew(env: JNIEnv, _: JObject) -> jobject {
  handle_exception_result(|| { 
    let result = Labels::new();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelsLen(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Labels>()?;
    let result = self_rptr.len();
    result.into_jlong().jobject(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelsGet(env: JNIEnv, _: JObject, self_ptr: JRPtr, index_jlong: jlong) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Labels>()?;
    let index = usize::try_from_jlong(index_jlong)?;
    let result = self_rptr.get(index);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1labelsAdd(env: JNIEnv, _: JObject, self_ptr: JRPtr, elem_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<Labels>()?;
    let elem_jrptr = elem_ptr.rptr(&env)?;
    let elem = elem_jrptr.typed_ref::<Label>()?;
    self_rptr.add(elem);
    Ok(JObject::null())
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1passwordEncryptionToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<PasswordEncryption>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1passwordEncryptionFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = PasswordEncryption::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1passwordEncryptionNew(env: JNIEnv, _: JObject, data_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let data_jrptr = data_ptr.rptr(&env)?;
    let data = data_jrptr.typed_ref::<COSEEncrypt0>()?;
    let result = PasswordEncryption::new(data);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1protectedHeaderMapToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<ProtectedHeaderMap>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1protectedHeaderMapFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = ProtectedHeaderMap::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1protectedHeaderMapNewEmpty(env: JNIEnv, _: JObject) -> jobject {
  handle_exception_result(|| { 
    let result = ProtectedHeaderMap::new_empty();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1protectedHeaderMapNew(env: JNIEnv, _: JObject, header_map_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let header_map_jrptr = header_map_ptr.rptr(&env)?;
    let header_map = header_map_jrptr.typed_ref::<HeaderMap>()?;
    let result = ProtectedHeaderMap::new(header_map);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1protectedHeaderMapDeserializedHeaders(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<ProtectedHeaderMap>()?;
    let result = self_rptr.deserialized_headers();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1pubKeyEncryptionToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<PubKeyEncryption>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1pubKeyEncryptionFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = PubKeyEncryption::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1pubKeyEncryptionNew(env: JNIEnv, _: JObject, data_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let data_jrptr = data_ptr.rptr(&env)?;
    let data = data_jrptr.typed_ref::<COSEEncrypt>()?;
    let result = PubKeyEncryption::new(data);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1sigStructureToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<SigStructure>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1sigStructureFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = SigStructure::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1sigStructureContext(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<SigStructure>()?;
    let result = self_rptr.context();
    (result.to_i32() as jint).jobject(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1sigStructureBodyProtected(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<SigStructure>()?;
    let result = self_rptr.body_protected();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1sigStructureSignProtected(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<SigStructure>()?;
    let result = self_rptr.sign_protected();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1sigStructureExternalAad(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<SigStructure>()?;
    let result = self_rptr.external_aad();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1sigStructurePayload(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<SigStructure>()?;
    let result = self_rptr.payload();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1sigStructureSetSignProtected(env: JNIEnv, _: JObject, self_ptr: JRPtr, sign_protected_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<SigStructure>()?;
    let sign_protected_jrptr = sign_protected_ptr.rptr(&env)?;
    let sign_protected = sign_protected_jrptr.typed_ref::<ProtectedHeaderMap>()?;
    self_rptr.set_sign_protected(sign_protected);
    Ok(JObject::null())
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1sigStructureNew(env: JNIEnv, _: JObject, context_jint: jint, body_protected_ptr: JRPtr, external_aad_jarray: jbyteArray, payload_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let context = context_jint.to_enum()?;
    let body_protected_jrptr = body_protected_ptr.rptr(&env)?;
    let body_protected = body_protected_jrptr.typed_ref::<ProtectedHeaderMap>()?;
    let external_aad = env.convert_byte_array(external_aad_jarray).into_result()?;
    let payload = env.convert_byte_array(payload_jarray).into_result()?;
    let result = SigStructure::new(context, body_protected, external_aad, payload);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1signedMessageToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<SignedMessage>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1signedMessageFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = SignedMessage::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1signedMessageNewCoseSign(env: JNIEnv, _: JObject, cose_sign_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let cose_sign_jrptr = cose_sign_ptr.rptr(&env)?;
    let cose_sign = cose_sign_jrptr.typed_ref::<COSESign>()?;
    let result = SignedMessage::new_cose_sign(cose_sign);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1signedMessageNewCoseSign1(env: JNIEnv, _: JObject, cose_sign1_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let cose_sign1_jrptr = cose_sign1_ptr.rptr(&env)?;
    let cose_sign1 = cose_sign1_jrptr.typed_ref::<COSESign1>()?;
    let result = SignedMessage::new_cose_sign1(cose_sign1);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1signedMessageFromUserFacingEncoding(env: JNIEnv, _: JObject, s_str: JString) -> jobject {
  handle_exception_result(|| { 
    let s = s_str.string(&env)?;
    let result = SignedMessage::from_user_facing_encoding(&s).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1signedMessageToUserFacingEncoding(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<SignedMessage>()?;
    let result = self_rptr.to_user_facing_encoding();
    result.jstring(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1signedMessageKind(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<SignedMessage>()?;
    let result = self_rptr.kind();
    (result.to_i32() as jint).jobject(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1signedMessageAsCoseSign(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<SignedMessage>()?;
    let result = self_rptr.as_cose_sign();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1signedMessageAsCoseSign1(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<SignedMessage>()?;
    let result = self_rptr.as_cose_sign1();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1taggedCBORToBytes(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<TaggedCBOR>()?;
    let result = self_rptr.to_bytes();
    Ok(JObject::from_raw(env.byte_array_from_slice(&result).into_result()?))
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1taggedCBORFromBytes(env: JNIEnv, _: JObject, bytes_jarray: jbyteArray) -> jobject {
  handle_exception_result(|| { 
    let bytes = env.convert_byte_array(bytes_jarray).into_result()?;
    let result = TaggedCBOR::from_bytes(bytes).into_result()?;
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1taggedCBORTag(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<TaggedCBOR>()?;
    let result = self_rptr.tag();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1taggedCBORValue(env: JNIEnv, _: JObject, self_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let self_jrptr = self_ptr.rptr(&env)?;
    let self_rptr = self_jrptr.typed_ref::<TaggedCBOR>()?;
    let result = self_rptr.value();
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnmslbridge_Native_msl_1bridge_1taggedCBORNew(env: JNIEnv, _: JObject, tag_ptr: JRPtr, value_ptr: JRPtr) -> jobject {
  handle_exception_result(|| { 
    let tag = tag_ptr.rptr(&env)?.typed_ref::<BigNum>()?.clone();
    let value_jrptr = value_ptr.rptr(&env)?;
    let value = value_jrptr.typed_ref::<CBORValue>()?;
    let result = TaggedCBOR::new(tag, value);
    result.rptr().jptr(&env)
  })
  .jresult(&env)
}



