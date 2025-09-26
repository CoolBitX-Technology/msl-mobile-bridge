/* eslint-disable max-len */
import { NativeModules } from 'react-native';
import { decode as base64_decode, encode as base64_encode } from 'base-64';

const { MessageSigningLib } = NativeModules;

// export default MessageSigningLib;

function uint8ArrayFromB64(base64_string) {
  if (base64_string == null) {
    return undefined;
  }
  return Uint8Array.from(base64_decode(base64_string), c => c.charCodeAt(0));
}

function b64FromUint8Array(uint8Array) {
  if (uint8Array == null) {
    return undefined;
  }
  return base64_encode(String.fromCharCode.apply(null, uint8Array));
}

function uint32ArrayToBase64(uint32Array) {
  if (uint32Array == null) {
    return undefined;
  }
  const uint8Array = new Uint8Array(uint32Array.length * 4);
  const dataView = new DataView(uint8Array.buffer);
  for (let i = 0; i < uint32Array.length; i++) {
    dataView.setUint32(i * 4, uint32Array[i], true);
  }
  return b64FromUint8Array(uint8Array);
}

function base64ToUint32Array(base64String) {
  if (base64String == null) {
    return undefined;
  }
  const uint8Array = uint8ArrayFromB64(base64String);
  const dataView = new DataView(uint8Array.buffer);
  const uint32Array = new Uint32Array(uint8Array.length / 4);
  for (let i = 0; i < uint32Array.length; i++) {
    uint32Array[i] = dataView.getUint32(i * 4, true);
  }
  return uint32Array;
}

class Ptr {
  static _wrap(ptr, klass) {
    if (ptr === '0' || ptr == null) {
      return undefined;
    }
    const obj = Object.create(klass.prototype);
    obj.ptr = ptr;
    return obj;
  }

  static _assertClass(ptr, klass) {
    if (!(ptr instanceof klass)) {
      throw new Error(`expected instance of ${klass.name}`);
    }
    return ptr.ptr;
  }

  static _assertOptionalClass(ptr, klass) {
    if (ptr == null) {
      return ptr;
    }
    if (!(ptr instanceof klass)) {
      throw new Error(`expected instance of ${klass.name}`);
    }
    return ptr.ptr;
  }

  constructor() {
    throw new Error("Can't be initialized with constructor");
  }

  /**
  * Frees the pointer
  * @returns {Promise<void>}
  */
  async free() {
    if (!this.ptr) {
      return;
    }
    const ptr = this.ptr;
    this.ptr = null;
    await MessageSigningLib.ptrFree(ptr);
  }
}

export class BigNum extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_bigNumToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_bigNumFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, BigNum);
  }

  static async from_str(string) {
    const ret = await MessageSigningLib.msl_bridge_bigNumFromStr(string);
    return Ptr._wrap(ret, BigNum);
  }

  async to_str() {
    const ret = await MessageSigningLib.msl_bridge_bigNumToStr(this.ptr);
    return ret;
  }

  async checked_mul(other) {
    const otherPtr = Ptr._assertClass(other, BigNum);
    const ret = await MessageSigningLib.msl_bridge_bigNumCheckedMul(this.ptr, otherPtr);
    return Ptr._wrap(ret, BigNum);
  }

  async checked_add(other) {
    const otherPtr = Ptr._assertClass(other, BigNum);
    const ret = await MessageSigningLib.msl_bridge_bigNumCheckedAdd(this.ptr, otherPtr);
    return Ptr._wrap(ret, BigNum);
  }

  async checked_sub(other) {
    const otherPtr = Ptr._assertClass(other, BigNum);
    const ret = await MessageSigningLib.msl_bridge_bigNumCheckedSub(this.ptr, otherPtr);
    return Ptr._wrap(ret, BigNum);
  }

}


export class CBORArray extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cBORArrayToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cBORArrayFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, CBORArray);
  }

  static async new() {
    const ret = await MessageSigningLib.msl_bridge_cBORArrayNew();
    return Ptr._wrap(ret, CBORArray);
  }

  async len() {
    const ret = await MessageSigningLib.msl_bridge_cBORArrayLen(this.ptr);
    return ret;
  }

  async get(index) {
    const ret = await MessageSigningLib.msl_bridge_cBORArrayGet(this.ptr, index);
    return Ptr._wrap(ret, CBORValue);
  }

  add(elem) {
    const elemPtr = Ptr._assertClass(elem, CBORValue);
    const ret = MessageSigningLib.msl_bridge_cBORArrayAdd(this.ptr, elemPtr);
    return ret;
  }

  set_definite_encoding(use_definite) {
    const ret = MessageSigningLib.msl_bridge_cBORArraySetDefiniteEncoding(this.ptr, use_definite);
    return ret;
  }

  async is_definite() {
    const ret = await MessageSigningLib.msl_bridge_cBORArrayIsDefinite(this.ptr);
    return ret;
  }

}


export class CBORObject extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cBORObjectToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cBORObjectFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, CBORObject);
  }

  static async new() {
    const ret = await MessageSigningLib.msl_bridge_cBORObjectNew();
    return Ptr._wrap(ret, CBORObject);
  }

  async len() {
    const ret = await MessageSigningLib.msl_bridge_cBORObjectLen(this.ptr);
    return ret;
  }

  async insert(key, value) {
    const keyPtr = Ptr._assertClass(key, CBORValue);
    const valuePtr = Ptr._assertClass(value, CBORValue);
    const ret = await MessageSigningLib.msl_bridge_cBORObjectInsert(this.ptr, keyPtr, valuePtr);
    return Ptr._wrap(ret, CBORValue);
  }

  async get(key) {
    const keyPtr = Ptr._assertClass(key, CBORValue);
    const ret = await MessageSigningLib.msl_bridge_cBORObjectGet(this.ptr, keyPtr);
    return Ptr._wrap(ret, CBORValue);
  }

  async keys() {
    const ret = await MessageSigningLib.msl_bridge_cBORObjectKeys(this.ptr);
    return Ptr._wrap(ret, CBORArray);
  }

  set_definite_encoding(use_definite) {
    const ret = MessageSigningLib.msl_bridge_cBORObjectSetDefiniteEncoding(this.ptr, use_definite);
    return ret;
  }

  async is_definite() {
    const ret = await MessageSigningLib.msl_bridge_cBORObjectIsDefinite(this.ptr);
    return ret;
  }

}


export class CBORSpecial extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cBORSpecialToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cBORSpecialFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, CBORSpecial);
  }

  static async new_bool(b) {
    const ret = await MessageSigningLib.msl_bridge_cBORSpecialNewBool(b);
    return Ptr._wrap(ret, CBORSpecial);
  }

  static async new_unassigned(u) {
    const ret = await MessageSigningLib.msl_bridge_cBORSpecialNewUnassigned(u);
    return Ptr._wrap(ret, CBORSpecial);
  }

  static async new_break() {
    const ret = await MessageSigningLib.msl_bridge_cBORSpecialNewBreak();
    return Ptr._wrap(ret, CBORSpecial);
  }

  static async new_null() {
    const ret = await MessageSigningLib.msl_bridge_cBORSpecialNewNull();
    return Ptr._wrap(ret, CBORSpecial);
  }

  static async new_undefined() {
    const ret = await MessageSigningLib.msl_bridge_cBORSpecialNewUndefined();
    return Ptr._wrap(ret, CBORSpecial);
  }

  async kind() {
    const ret = await MessageSigningLib.msl_bridge_cBORSpecialKind(this.ptr);
    return ret;
  }

  async as_bool() {
    const ret = await MessageSigningLib.msl_bridge_cBORSpecialAsBool(this.ptr);
    return ret;
  }

  async as_float() {
    const ret = await MessageSigningLib.msl_bridge_cBORSpecialAsFloat(this.ptr);
    return ret;
  }

  async as_unassigned() {
    const ret = await MessageSigningLib.msl_bridge_cBORSpecialAsUnassigned(this.ptr);
    return ret;
  }

}


export class CBORValue extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cBORValueToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cBORValueFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, CBORValue);
  }

  static async new_int(int_value) {
    const int_valuePtr = Ptr._assertClass(int_value, Int);
    const ret = await MessageSigningLib.msl_bridge_cBORValueNewInt(int_valuePtr);
    return Ptr._wrap(ret, CBORValue);
  }

  static async new_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cBORValueNewBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, CBORValue);
  }

  static async new_text(text) {
    const ret = await MessageSigningLib.msl_bridge_cBORValueNewText(text);
    return Ptr._wrap(ret, CBORValue);
  }

  static async new_array(arr) {
    const arrPtr = Ptr._assertClass(arr, CBORArray);
    const ret = await MessageSigningLib.msl_bridge_cBORValueNewArray(arrPtr);
    return Ptr._wrap(ret, CBORValue);
  }

  static async new_object(obj) {
    const objPtr = Ptr._assertClass(obj, CBORObject);
    const ret = await MessageSigningLib.msl_bridge_cBORValueNewObject(objPtr);
    return Ptr._wrap(ret, CBORValue);
  }

  static async new_tagged(tagged) {
    const taggedPtr = Ptr._assertClass(tagged, TaggedCBOR);
    const ret = await MessageSigningLib.msl_bridge_cBORValueNewTagged(taggedPtr);
    return Ptr._wrap(ret, CBORValue);
  }

  static async new_special(special) {
    const specialPtr = Ptr._assertClass(special, CBORSpecial);
    const ret = await MessageSigningLib.msl_bridge_cBORValueNewSpecial(specialPtr);
    return Ptr._wrap(ret, CBORValue);
  }

  static async from_label(label) {
    const labelPtr = Ptr._assertClass(label, Label);
    const ret = await MessageSigningLib.msl_bridge_cBORValueFromLabel(labelPtr);
    return Ptr._wrap(ret, CBORValue);
  }

  async kind() {
    const ret = await MessageSigningLib.msl_bridge_cBORValueKind(this.ptr);
    return ret;
  }

  async as_int() {
    const ret = await MessageSigningLib.msl_bridge_cBORValueAsInt(this.ptr);
    return Ptr._wrap(ret, Int);
  }

  async as_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cBORValueAsBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  async as_text() {
    const ret = await MessageSigningLib.msl_bridge_cBORValueAsText(this.ptr);
    return ret;
  }

  async as_array() {
    const ret = await MessageSigningLib.msl_bridge_cBORValueAsArray(this.ptr);
    return Ptr._wrap(ret, CBORArray);
  }

  async as_object() {
    const ret = await MessageSigningLib.msl_bridge_cBORValueAsObject(this.ptr);
    return Ptr._wrap(ret, CBORObject);
  }

  async as_tagged() {
    const ret = await MessageSigningLib.msl_bridge_cBORValueAsTagged(this.ptr);
    return Ptr._wrap(ret, TaggedCBOR);
  }

  async as_special() {
    const ret = await MessageSigningLib.msl_bridge_cBORValueAsSpecial(this.ptr);
    return Ptr._wrap(ret, CBORSpecial);
  }

}


export class COSEEncrypt extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cOSEEncryptToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cOSEEncryptFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, COSEEncrypt);
  }

  async headers() {
    const ret = await MessageSigningLib.msl_bridge_cOSEEncryptHeaders(this.ptr);
    return Ptr._wrap(ret, Headers);
  }

  async ciphertext() {
    const ret = await MessageSigningLib.msl_bridge_cOSEEncryptCiphertext(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  async recipients() {
    const ret = await MessageSigningLib.msl_bridge_cOSEEncryptRecipients(this.ptr);
    return Ptr._wrap(ret, COSERecipients);
  }

  static async new(headers, ciphertext, recipients) {
    const headersPtr = Ptr._assertClass(headers, Headers);
    const recipientsPtr = Ptr._assertClass(recipients, COSERecipients);
    if(ciphertext == null) {
      const ret = await MessageSigningLib.msl_bridge_cOSEEncryptNew(headersPtr, recipientsPtr);
      return Ptr._wrap(ret, COSEEncrypt);
    }
    if(ciphertext != null) {
      const ret = await MessageSigningLib.msl_bridge_cOSEEncryptNewWithCiphertext(headersPtr, b64FromUint8Array(ciphertext), recipientsPtr);
      return Ptr._wrap(ret, COSEEncrypt);
    }
  }

}


export class COSEEncrypt0 extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cOSEEncrypt0ToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cOSEEncrypt0FromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, COSEEncrypt0);
  }

  async headers() {
    const ret = await MessageSigningLib.msl_bridge_cOSEEncrypt0Headers(this.ptr);
    return Ptr._wrap(ret, Headers);
  }

  async ciphertext() {
    const ret = await MessageSigningLib.msl_bridge_cOSEEncrypt0Ciphertext(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async new(headers, ciphertext) {
    const headersPtr = Ptr._assertClass(headers, Headers);
    if(ciphertext == null) {
      const ret = await MessageSigningLib.msl_bridge_cOSEEncrypt0New(headersPtr);
      return Ptr._wrap(ret, COSEEncrypt0);
    }
    if(ciphertext != null) {
      const ret = await MessageSigningLib.msl_bridge_cOSEEncrypt0NewWithCiphertext(headersPtr, b64FromUint8Array(ciphertext));
      return Ptr._wrap(ret, COSEEncrypt0);
    }
  }

}


export class COSEKey extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cOSEKeyToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cOSEKeyFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, COSEKey);
  }

  set_key_type(key_type) {
    const key_typePtr = Ptr._assertClass(key_type, Label);
    const ret = MessageSigningLib.msl_bridge_cOSEKeySetKeyType(this.ptr, key_typePtr);
    return ret;
  }

  async key_type() {
    const ret = await MessageSigningLib.msl_bridge_cOSEKeyKeyType(this.ptr);
    return Ptr._wrap(ret, Label);
  }

  set_key_id(key_id) {
    const ret = MessageSigningLib.msl_bridge_cOSEKeySetKeyId(this.ptr, b64FromUint8Array(key_id));
    return ret;
  }

  async key_id() {
    const ret = await MessageSigningLib.msl_bridge_cOSEKeyKeyId(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  set_algorithm_id(algorithm_id) {
    const algorithm_idPtr = Ptr._assertClass(algorithm_id, Label);
    const ret = MessageSigningLib.msl_bridge_cOSEKeySetAlgorithmId(this.ptr, algorithm_idPtr);
    return ret;
  }

  async algorithm_id() {
    const ret = await MessageSigningLib.msl_bridge_cOSEKeyAlgorithmId(this.ptr);
    return Ptr._wrap(ret, Label);
  }

  set_key_ops(key_ops) {
    const key_opsPtr = Ptr._assertClass(key_ops, Labels);
    const ret = MessageSigningLib.msl_bridge_cOSEKeySetKeyOps(this.ptr, key_opsPtr);
    return ret;
  }

  async key_ops() {
    const ret = await MessageSigningLib.msl_bridge_cOSEKeyKeyOps(this.ptr);
    return Ptr._wrap(ret, Labels);
  }

  set_base_init_vector(base_init_vector) {
    const ret = MessageSigningLib.msl_bridge_cOSEKeySetBaseInitVector(this.ptr, b64FromUint8Array(base_init_vector));
    return ret;
  }

  async base_init_vector() {
    const ret = await MessageSigningLib.msl_bridge_cOSEKeyBaseInitVector(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  async header(label) {
    const labelPtr = Ptr._assertClass(label, Label);
    const ret = await MessageSigningLib.msl_bridge_cOSEKeyHeader(this.ptr, labelPtr);
    return Ptr._wrap(ret, CBORValue);
  }

  set_header(label, value) {
    const labelPtr = Ptr._assertClass(label, Label);
    const valuePtr = Ptr._assertClass(value, CBORValue);
    const ret = MessageSigningLib.msl_bridge_cOSEKeySetHeader(this.ptr, labelPtr, valuePtr);
    return ret;
  }

  static async new(key_type) {
    const key_typePtr = Ptr._assertClass(key_type, Label);
    const ret = await MessageSigningLib.msl_bridge_cOSEKeyNew(key_typePtr);
    return Ptr._wrap(ret, COSEKey);
  }

}


export class COSERecipient extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cOSERecipientToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cOSERecipientFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, COSERecipient);
  }

  async headers() {
    const ret = await MessageSigningLib.msl_bridge_cOSERecipientHeaders(this.ptr);
    return Ptr._wrap(ret, Headers);
  }

  async ciphertext() {
    const ret = await MessageSigningLib.msl_bridge_cOSERecipientCiphertext(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async new(headers, ciphertext) {
    const headersPtr = Ptr._assertClass(headers, Headers);
    if(ciphertext == null) {
      const ret = await MessageSigningLib.msl_bridge_cOSERecipientNew(headersPtr);
      return Ptr._wrap(ret, COSERecipient);
    }
    if(ciphertext != null) {
      const ret = await MessageSigningLib.msl_bridge_cOSERecipientNewWithCiphertext(headersPtr, b64FromUint8Array(ciphertext));
      return Ptr._wrap(ret, COSERecipient);
    }
  }

}


export class COSERecipients extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cOSERecipientsToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cOSERecipientsFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, COSERecipients);
  }

  static async new() {
    const ret = await MessageSigningLib.msl_bridge_cOSERecipientsNew();
    return Ptr._wrap(ret, COSERecipients);
  }

  async len() {
    const ret = await MessageSigningLib.msl_bridge_cOSERecipientsLen(this.ptr);
    return ret;
  }

  async get(index) {
    const ret = await MessageSigningLib.msl_bridge_cOSERecipientsGet(this.ptr, index);
    return Ptr._wrap(ret, COSERecipient);
  }

  add(elem) {
    const elemPtr = Ptr._assertClass(elem, COSERecipient);
    const ret = MessageSigningLib.msl_bridge_cOSERecipientsAdd(this.ptr, elemPtr);
    return ret;
  }

}


export class COSESign extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cOSESignToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cOSESignFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, COSESign);
  }

  async headers() {
    const ret = await MessageSigningLib.msl_bridge_cOSESignHeaders(this.ptr);
    return Ptr._wrap(ret, Headers);
  }

  async payload() {
    const ret = await MessageSigningLib.msl_bridge_cOSESignPayload(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  async signatures() {
    const ret = await MessageSigningLib.msl_bridge_cOSESignSignatures(this.ptr);
    return Ptr._wrap(ret, COSESignatures);
  }

  static async new(headers, payload, signatures) {
    const headersPtr = Ptr._assertClass(headers, Headers);
    const signaturesPtr = Ptr._assertClass(signatures, COSESignatures);
    if(payload == null) {
      const ret = await MessageSigningLib.msl_bridge_cOSESignNew(headersPtr, signaturesPtr);
      return Ptr._wrap(ret, COSESign);
    }
    if(payload != null) {
      const ret = await MessageSigningLib.msl_bridge_cOSESignNewWithPayload(headersPtr, b64FromUint8Array(payload), signaturesPtr);
      return Ptr._wrap(ret, COSESign);
    }
  }

}


export class COSESign1 extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cOSESign1ToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cOSESign1FromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, COSESign1);
  }

  async headers() {
    const ret = await MessageSigningLib.msl_bridge_cOSESign1Headers(this.ptr);
    return Ptr._wrap(ret, Headers);
  }

  async payload() {
    const ret = await MessageSigningLib.msl_bridge_cOSESign1Payload(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  async signature() {
    const ret = await MessageSigningLib.msl_bridge_cOSESign1Signature(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  async signed_data(external_aad, external_payload) {
    if(external_aad == null && external_payload == null) {
      const ret = await MessageSigningLib.msl_bridge_cOSESign1SignedData(this.ptr);
      return Ptr._wrap(ret, SigStructure);
    }
    if(external_aad != null && external_payload == null) {
      const ret = await MessageSigningLib.msl_bridge_cOSESign1SignedDataWithExternalAad(this.ptr, b64FromUint8Array(external_aad));
      return Ptr._wrap(ret, SigStructure);
    }
    if(external_aad == null && external_payload != null) {
      const ret = await MessageSigningLib.msl_bridge_cOSESign1SignedDataWithExternalPayload(this.ptr, b64FromUint8Array(external_payload));
      return Ptr._wrap(ret, SigStructure);
    }
    if(external_aad != null && external_payload != null) {
      const ret = await MessageSigningLib.msl_bridge_cOSESign1SignedDataWithExternalAadExternalPayload(this.ptr, b64FromUint8Array(external_aad), b64FromUint8Array(external_payload));
      return Ptr._wrap(ret, SigStructure);
    }
  }

  static async new(headers, payload, signature) {
    const headersPtr = Ptr._assertClass(headers, Headers);
    if(payload == null) {
      const ret = await MessageSigningLib.msl_bridge_cOSESign1New(headersPtr, b64FromUint8Array(signature));
      return Ptr._wrap(ret, COSESign1);
    }
    if(payload != null) {
      const ret = await MessageSigningLib.msl_bridge_cOSESign1NewWithPayload(headersPtr, b64FromUint8Array(payload), b64FromUint8Array(signature));
      return Ptr._wrap(ret, COSESign1);
    }
  }

}


export class COSESign1Builder extends Ptr {
  static async new(headers, payload, is_payload_external) {
    const headersPtr = Ptr._assertClass(headers, Headers);
    const ret = await MessageSigningLib.msl_bridge_cOSESign1BuilderNew(headersPtr, b64FromUint8Array(payload), is_payload_external);
    return Ptr._wrap(ret, COSESign1Builder);
  }

  hash_payload() {
    const ret = MessageSigningLib.msl_bridge_cOSESign1BuilderHashPayload(this.ptr);
    return ret;
  }

  set_external_aad(external_aad) {
    const ret = MessageSigningLib.msl_bridge_cOSESign1BuilderSetExternalAad(this.ptr, b64FromUint8Array(external_aad));
    return ret;
  }

  async make_data_to_sign() {
    const ret = await MessageSigningLib.msl_bridge_cOSESign1BuilderMakeDataToSign(this.ptr);
    return Ptr._wrap(ret, SigStructure);
  }

  async build(signed_sig_structure) {
    const ret = await MessageSigningLib.msl_bridge_cOSESign1BuilderBuild(this.ptr, b64FromUint8Array(signed_sig_structure));
    return Ptr._wrap(ret, COSESign1);
  }

}


export class COSESignBuilder extends Ptr {
  static async new(headers, payload, is_payload_external) {
    const headersPtr = Ptr._assertClass(headers, Headers);
    const ret = await MessageSigningLib.msl_bridge_cOSESignBuilderNew(headersPtr, b64FromUint8Array(payload), is_payload_external);
    return Ptr._wrap(ret, COSESignBuilder);
  }

  hash_payload() {
    const ret = MessageSigningLib.msl_bridge_cOSESignBuilderHashPayload(this.ptr);
    return ret;
  }

  set_external_aad(external_aad) {
    const ret = MessageSigningLib.msl_bridge_cOSESignBuilderSetExternalAad(this.ptr, b64FromUint8Array(external_aad));
    return ret;
  }

  async make_data_to_sign() {
    const ret = await MessageSigningLib.msl_bridge_cOSESignBuilderMakeDataToSign(this.ptr);
    return Ptr._wrap(ret, SigStructure);
  }

  async build(signed_sig_structure) {
    const signed_sig_structurePtr = Ptr._assertClass(signed_sig_structure, COSESignatures);
    const ret = await MessageSigningLib.msl_bridge_cOSESignBuilderBuild(this.ptr, signed_sig_structurePtr);
    return Ptr._wrap(ret, COSESign);
  }

}


export class COSESignature extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cOSESignatureToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cOSESignatureFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, COSESignature);
  }

  async headers() {
    const ret = await MessageSigningLib.msl_bridge_cOSESignatureHeaders(this.ptr);
    return Ptr._wrap(ret, Headers);
  }

  async signature() {
    const ret = await MessageSigningLib.msl_bridge_cOSESignatureSignature(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async new(headers, signature) {
    const headersPtr = Ptr._assertClass(headers, Headers);
    const ret = await MessageSigningLib.msl_bridge_cOSESignatureNew(headersPtr, b64FromUint8Array(signature));
    return Ptr._wrap(ret, COSESignature);
  }

}


export class COSESignatures extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_cOSESignaturesToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_cOSESignaturesFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, COSESignatures);
  }

  static async new() {
    const ret = await MessageSigningLib.msl_bridge_cOSESignaturesNew();
    return Ptr._wrap(ret, COSESignatures);
  }

  async len() {
    const ret = await MessageSigningLib.msl_bridge_cOSESignaturesLen(this.ptr);
    return ret;
  }

  async get(index) {
    const ret = await MessageSigningLib.msl_bridge_cOSESignaturesGet(this.ptr, index);
    return Ptr._wrap(ret, COSESignature);
  }

  add(elem) {
    const elemPtr = Ptr._assertClass(elem, COSESignature);
    const ret = MessageSigningLib.msl_bridge_cOSESignaturesAdd(this.ptr, elemPtr);
    return ret;
  }

}


export class CounterSignature extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_counterSignatureToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_counterSignatureFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, CounterSignature);
  }

  static async new_single(cose_signature) {
    const cose_signaturePtr = Ptr._assertClass(cose_signature, COSESignature);
    const ret = await MessageSigningLib.msl_bridge_counterSignatureNewSingle(cose_signaturePtr);
    return Ptr._wrap(ret, CounterSignature);
  }

  static async new_multi(cose_signatures) {
    const cose_signaturesPtr = Ptr._assertClass(cose_signatures, COSESignatures);
    const ret = await MessageSigningLib.msl_bridge_counterSignatureNewMulti(cose_signaturesPtr);
    return Ptr._wrap(ret, CounterSignature);
  }

  async signatures() {
    const ret = await MessageSigningLib.msl_bridge_counterSignatureSignatures(this.ptr);
    return Ptr._wrap(ret, COSESignatures);
  }

}


export class EdDSA25519Key extends Ptr {
  static async new(pubkey_bytes) {
    const ret = await MessageSigningLib.msl_bridge_edDSA25519KeyNew(b64FromUint8Array(pubkey_bytes));
    return Ptr._wrap(ret, EdDSA25519Key);
  }

  set_private_key(private_key_bytes) {
    const ret = MessageSigningLib.msl_bridge_edDSA25519KeySetPrivateKey(this.ptr, b64FromUint8Array(private_key_bytes));
    return ret;
  }

  is_for_signing() {
    const ret = MessageSigningLib.msl_bridge_edDSA25519KeyIsForSigning(this.ptr);
    return ret;
  }

  is_for_verifying() {
    const ret = MessageSigningLib.msl_bridge_edDSA25519KeyIsForVerifying(this.ptr);
    return ret;
  }

  async build() {
    const ret = await MessageSigningLib.msl_bridge_edDSA25519KeyBuild(this.ptr);
    return Ptr._wrap(ret, COSEKey);
  }

}


export class HeaderMap extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_headerMapToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_headerMapFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, HeaderMap);
  }

  set_algorithm_id(algorithm_id) {
    const algorithm_idPtr = Ptr._assertClass(algorithm_id, Label);
    const ret = MessageSigningLib.msl_bridge_headerMapSetAlgorithmId(this.ptr, algorithm_idPtr);
    return ret;
  }

  async algorithm_id() {
    const ret = await MessageSigningLib.msl_bridge_headerMapAlgorithmId(this.ptr);
    return Ptr._wrap(ret, Label);
  }

  set_criticality(criticality) {
    const criticalityPtr = Ptr._assertClass(criticality, Labels);
    const ret = MessageSigningLib.msl_bridge_headerMapSetCriticality(this.ptr, criticalityPtr);
    return ret;
  }

  async criticality() {
    const ret = await MessageSigningLib.msl_bridge_headerMapCriticality(this.ptr);
    return Ptr._wrap(ret, Labels);
  }

  set_content_type(content_type) {
    const content_typePtr = Ptr._assertClass(content_type, Label);
    const ret = MessageSigningLib.msl_bridge_headerMapSetContentType(this.ptr, content_typePtr);
    return ret;
  }

  async content_type() {
    const ret = await MessageSigningLib.msl_bridge_headerMapContentType(this.ptr);
    return Ptr._wrap(ret, Label);
  }

  set_key_id(key_id) {
    const ret = MessageSigningLib.msl_bridge_headerMapSetKeyId(this.ptr, b64FromUint8Array(key_id));
    return ret;
  }

  async key_id() {
    const ret = await MessageSigningLib.msl_bridge_headerMapKeyId(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  set_init_vector(init_vector) {
    const ret = MessageSigningLib.msl_bridge_headerMapSetInitVector(this.ptr, b64FromUint8Array(init_vector));
    return ret;
  }

  async init_vector() {
    const ret = await MessageSigningLib.msl_bridge_headerMapInitVector(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  set_partial_init_vector(partial_init_vector) {
    const ret = MessageSigningLib.msl_bridge_headerMapSetPartialInitVector(this.ptr, b64FromUint8Array(partial_init_vector));
    return ret;
  }

  async partial_init_vector() {
    const ret = await MessageSigningLib.msl_bridge_headerMapPartialInitVector(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  set_counter_signature(counter_signature) {
    const counter_signaturePtr = Ptr._assertClass(counter_signature, CounterSignature);
    const ret = MessageSigningLib.msl_bridge_headerMapSetCounterSignature(this.ptr, counter_signaturePtr);
    return ret;
  }

  async counter_signature() {
    const ret = await MessageSigningLib.msl_bridge_headerMapCounterSignature(this.ptr);
    return Ptr._wrap(ret, CounterSignature);
  }

  async header(label) {
    const labelPtr = Ptr._assertClass(label, Label);
    const ret = await MessageSigningLib.msl_bridge_headerMapHeader(this.ptr, labelPtr);
    return Ptr._wrap(ret, CBORValue);
  }

  set_header(label, value) {
    const labelPtr = Ptr._assertClass(label, Label);
    const valuePtr = Ptr._assertClass(value, CBORValue);
    const ret = MessageSigningLib.msl_bridge_headerMapSetHeader(this.ptr, labelPtr, valuePtr);
    return ret;
  }

  async keys() {
    const ret = await MessageSigningLib.msl_bridge_headerMapKeys(this.ptr);
    return Ptr._wrap(ret, Labels);
  }

  static async new() {
    const ret = await MessageSigningLib.msl_bridge_headerMapNew();
    return Ptr._wrap(ret, HeaderMap);
  }

}


export class Headers extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_headersToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_headersFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, Headers);
  }

  async protected() {
    const ret = await MessageSigningLib.msl_bridge_headersProtected(this.ptr);
    return Ptr._wrap(ret, ProtectedHeaderMap);
  }

  async unprotected() {
    const ret = await MessageSigningLib.msl_bridge_headersUnprotected(this.ptr);
    return Ptr._wrap(ret, HeaderMap);
  }

  static async new(protected_, unprotected_) {
    const protected_Ptr = Ptr._assertClass(protected_, ProtectedHeaderMap);
    const unprotected_Ptr = Ptr._assertClass(unprotected_, HeaderMap);
    const ret = await MessageSigningLib.msl_bridge_headersNew(protected_Ptr, unprotected_Ptr);
    return Ptr._wrap(ret, Headers);
  }

}


export class Int extends Ptr {
  static async new(x) {
    const xPtr = Ptr._assertClass(x, BigNum);
    const ret = await MessageSigningLib.msl_bridge_intNew(xPtr);
    return Ptr._wrap(ret, Int);
  }

  static async new_negative(x) {
    const xPtr = Ptr._assertClass(x, BigNum);
    const ret = await MessageSigningLib.msl_bridge_intNewNegative(xPtr);
    return Ptr._wrap(ret, Int);
  }

  static async new_i32(x) {
    const ret = await MessageSigningLib.msl_bridge_intNewI32(x);
    return Ptr._wrap(ret, Int);
  }

  async is_positive() {
    const ret = await MessageSigningLib.msl_bridge_intIsPositive(this.ptr);
    return ret;
  }

  async as_positive() {
    const ret = await MessageSigningLib.msl_bridge_intAsPositive(this.ptr);
    return Ptr._wrap(ret, BigNum);
  }

  async as_negative() {
    const ret = await MessageSigningLib.msl_bridge_intAsNegative(this.ptr);
    return Ptr._wrap(ret, BigNum);
  }

  async as_i32() {
    const ret = await MessageSigningLib.msl_bridge_intAsI32(this.ptr);
    return ret;
  }

}


export class Label extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_labelToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_labelFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, Label);
  }

  static async new_int(int_value) {
    const int_valuePtr = Ptr._assertClass(int_value, Int);
    const ret = await MessageSigningLib.msl_bridge_labelNewInt(int_valuePtr);
    return Ptr._wrap(ret, Label);
  }

  static async new_text(text) {
    const ret = await MessageSigningLib.msl_bridge_labelNewText(text);
    return Ptr._wrap(ret, Label);
  }

  async kind() {
    const ret = await MessageSigningLib.msl_bridge_labelKind(this.ptr);
    return ret;
  }

  async as_int() {
    const ret = await MessageSigningLib.msl_bridge_labelAsInt(this.ptr);
    return Ptr._wrap(ret, Int);
  }

  async as_text() {
    const ret = await MessageSigningLib.msl_bridge_labelAsText(this.ptr);
    return ret;
  }

  static async from_algorithm_id(id) {
    const ret = await MessageSigningLib.msl_bridge_labelFromAlgorithmId(id);
    return Ptr._wrap(ret, Label);
  }

  static async from_key_type(key_type) {
    const ret = await MessageSigningLib.msl_bridge_labelFromKeyType(key_type);
    return Ptr._wrap(ret, Label);
  }

  static async from_ec_key(ec_key) {
    const ret = await MessageSigningLib.msl_bridge_labelFromEcKey(ec_key);
    return Ptr._wrap(ret, Label);
  }

  static async from_curve_type(curve_type) {
    const ret = await MessageSigningLib.msl_bridge_labelFromCurveType(curve_type);
    return Ptr._wrap(ret, Label);
  }

  static async from_key_operation(key_op) {
    const ret = await MessageSigningLib.msl_bridge_labelFromKeyOperation(key_op);
    return Ptr._wrap(ret, Label);
  }

}


export class Labels extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_labelsToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_labelsFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, Labels);
  }

  static async new() {
    const ret = await MessageSigningLib.msl_bridge_labelsNew();
    return Ptr._wrap(ret, Labels);
  }

  async len() {
    const ret = await MessageSigningLib.msl_bridge_labelsLen(this.ptr);
    return ret;
  }

  async get(index) {
    const ret = await MessageSigningLib.msl_bridge_labelsGet(this.ptr, index);
    return Ptr._wrap(ret, Label);
  }

  add(elem) {
    const elemPtr = Ptr._assertClass(elem, Label);
    const ret = MessageSigningLib.msl_bridge_labelsAdd(this.ptr, elemPtr);
    return ret;
  }

}


export class PasswordEncryption extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_passwordEncryptionToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_passwordEncryptionFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, PasswordEncryption);
  }

  static async new(data) {
    const dataPtr = Ptr._assertClass(data, COSEEncrypt0);
    const ret = await MessageSigningLib.msl_bridge_passwordEncryptionNew(dataPtr);
    return Ptr._wrap(ret, PasswordEncryption);
  }

}


export class ProtectedHeaderMap extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_protectedHeaderMapToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_protectedHeaderMapFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, ProtectedHeaderMap);
  }

  static async new_empty() {
    const ret = await MessageSigningLib.msl_bridge_protectedHeaderMapNewEmpty();
    return Ptr._wrap(ret, ProtectedHeaderMap);
  }

  static async new(header_map) {
    const header_mapPtr = Ptr._assertClass(header_map, HeaderMap);
    const ret = await MessageSigningLib.msl_bridge_protectedHeaderMapNew(header_mapPtr);
    return Ptr._wrap(ret, ProtectedHeaderMap);
  }

  async deserialized_headers() {
    const ret = await MessageSigningLib.msl_bridge_protectedHeaderMapDeserializedHeaders(this.ptr);
    return Ptr._wrap(ret, HeaderMap);
  }

}


export class PubKeyEncryption extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_pubKeyEncryptionToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_pubKeyEncryptionFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, PubKeyEncryption);
  }

  static async new(data) {
    const dataPtr = Ptr._assertClass(data, COSEEncrypt);
    const ret = await MessageSigningLib.msl_bridge_pubKeyEncryptionNew(dataPtr);
    return Ptr._wrap(ret, PubKeyEncryption);
  }

}


export class SigStructure extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_sigStructureToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_sigStructureFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, SigStructure);
  }

  async context() {
    const ret = await MessageSigningLib.msl_bridge_sigStructureContext(this.ptr);
    return ret;
  }

  async body_protected() {
    const ret = await MessageSigningLib.msl_bridge_sigStructureBodyProtected(this.ptr);
    return Ptr._wrap(ret, ProtectedHeaderMap);
  }

  async sign_protected() {
    const ret = await MessageSigningLib.msl_bridge_sigStructureSignProtected(this.ptr);
    return Ptr._wrap(ret, ProtectedHeaderMap);
  }

  async external_aad() {
    const ret = await MessageSigningLib.msl_bridge_sigStructureExternalAad(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  async payload() {
    const ret = await MessageSigningLib.msl_bridge_sigStructurePayload(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  set_sign_protected(sign_protected) {
    const sign_protectedPtr = Ptr._assertClass(sign_protected, ProtectedHeaderMap);
    const ret = MessageSigningLib.msl_bridge_sigStructureSetSignProtected(this.ptr, sign_protectedPtr);
    return ret;
  }

  static async new(context, body_protected, external_aad, payload) {
    const body_protectedPtr = Ptr._assertClass(body_protected, ProtectedHeaderMap);
    const ret = await MessageSigningLib.msl_bridge_sigStructureNew(context, body_protectedPtr, b64FromUint8Array(external_aad), b64FromUint8Array(payload));
    return Ptr._wrap(ret, SigStructure);
  }

}


export class SignedMessage extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_signedMessageToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_signedMessageFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, SignedMessage);
  }

  static async new_cose_sign(cose_sign) {
    const cose_signPtr = Ptr._assertClass(cose_sign, COSESign);
    const ret = await MessageSigningLib.msl_bridge_signedMessageNewCoseSign(cose_signPtr);
    return Ptr._wrap(ret, SignedMessage);
  }

  static async new_cose_sign1(cose_sign1) {
    const cose_sign1Ptr = Ptr._assertClass(cose_sign1, COSESign1);
    const ret = await MessageSigningLib.msl_bridge_signedMessageNewCoseSign1(cose_sign1Ptr);
    return Ptr._wrap(ret, SignedMessage);
  }

  static async from_user_facing_encoding(s) {
    const ret = await MessageSigningLib.msl_bridge_signedMessageFromUserFacingEncoding(s);
    return Ptr._wrap(ret, SignedMessage);
  }

  async to_user_facing_encoding() {
    const ret = await MessageSigningLib.msl_bridge_signedMessageToUserFacingEncoding(this.ptr);
    return ret;
  }

  async kind() {
    const ret = await MessageSigningLib.msl_bridge_signedMessageKind(this.ptr);
    return ret;
  }

  async as_cose_sign() {
    const ret = await MessageSigningLib.msl_bridge_signedMessageAsCoseSign(this.ptr);
    return Ptr._wrap(ret, COSESign);
  }

  async as_cose_sign1() {
    const ret = await MessageSigningLib.msl_bridge_signedMessageAsCoseSign1(this.ptr);
    return Ptr._wrap(ret, COSESign1);
  }

}


export class TaggedCBOR extends Ptr {
  async to_bytes() {
    const ret = await MessageSigningLib.msl_bridge_taggedCBORToBytes(this.ptr);
    return uint8ArrayFromB64(ret);
  }

  static async from_bytes(bytes) {
    const ret = await MessageSigningLib.msl_bridge_taggedCBORFromBytes(b64FromUint8Array(bytes));
    return Ptr._wrap(ret, TaggedCBOR);
  }

  async tag() {
    const ret = await MessageSigningLib.msl_bridge_taggedCBORTag(this.ptr);
    return Ptr._wrap(ret, BigNum);
  }

  async value() {
    const ret = await MessageSigningLib.msl_bridge_taggedCBORValue(this.ptr);
    return Ptr._wrap(ret, CBORValue);
  }

  static async new(tag, value) {
    const tagPtr = Ptr._assertClass(tag, BigNum);
    const valuePtr = Ptr._assertClass(value, CBORValue);
    const ret = await MessageSigningLib.msl_bridge_taggedCBORNew(tagPtr, valuePtr);
    return Ptr._wrap(ret, TaggedCBOR);
  }

}


export const AlgorithmId = Object.freeze({
  EdDSA: 0,
  ChaCha20Poly1305: 1,
});


export const CBORSpecialType = Object.freeze({
  Bool: 0,
  Float: 1,
  Unassigned: 2,
  Break: 3,
  Undefined: 4,
  Null: 5,
});


export const CBORValueKind = Object.freeze({
  Int: 0,
  Bytes: 1,
  Text: 2,
  Array: 3,
  Object: 4,
  TaggedCBOR: 5,
  Special: 6,
});


export const CurveType = Object.freeze({
  P256: 0,
  P384: 1,
  P521: 2,
  X25519: 3,
  X448: 4,
  Ed25519: 5,
  Ed448: 6,
});


export const ECKey = Object.freeze({
  CRV: 0,
  X: 1,
  Y: 2,
  D: 3,
});


export const KeyOperation = Object.freeze({
  Sign: 0,
  Verify: 1,
  Encrypt: 2,
  Decrypt: 3,
  WrapKey: 4,
  UnwrapKey: 5,
  DeriveKey: 6,
  DeriveBits: 7,
});


export const KeyType = Object.freeze({
  OKP: 0,
  EC2: 1,
  Symmetric: 2,
});


export const LabelKind = Object.freeze({
  Int: 0,
  Text: 1,
});


export const SigContext = Object.freeze({
  Signature: 0,
  Signature1: 1,
  CounterSignature: 2,
});


export const SignedMessageKind = Object.freeze({
  COSESIGN: 0,
  COSESIGN1: 1,
});


