package io.emurgo.rnmslbridge;

import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;

import android.util.Base64;
import java.util.HashMap;
import java.util.Map;

public class MessageSigningLibModule extends ReactContextBaseJavaModule {

    private final ReactApplicationContext reactContext;

    public MessageSigningLibModule(ReactApplicationContext reactContext) {
        super(reactContext);
        this.reactContext = reactContext;
    }

    @Override
    public String getName() {
        return "MessageSigningLib";
    }

    @ReactMethod
    public final void ptrFree(String ptr, Promise promise) {
        try {
            (new RPtr(ptr)).free();
            promise.resolve(null);
        } catch (Throwable err) {
            promise.reject(err);
        }
    }
    @ReactMethod
    public final void msl_bridge_bigNumToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_bigNumToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_bigNumFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_bigNumFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_bigNumFromStr(String string, Promise promise) {
        Native.I
            .msl_bridge_bigNumFromStr(string)
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_bigNumToStr(String self, Promise promise) {
        Native.I
            .msl_bridge_bigNumToStr(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_bigNumCheckedMul(String self, String other, Promise promise) {
        Native.I
            .msl_bridge_bigNumCheckedMul(new RPtr(self), new RPtr(other))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_bigNumCheckedAdd(String self, String other, Promise promise) {
        Native.I
            .msl_bridge_bigNumCheckedAdd(new RPtr(self), new RPtr(other))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_bigNumCheckedSub(String self, String other, Promise promise) {
        Native.I
            .msl_bridge_bigNumCheckedSub(new RPtr(self), new RPtr(other))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_cBORArrayToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORArrayToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORArrayFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cBORArrayFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORArrayNew( Promise promise) {
        Native.I
            .msl_bridge_cBORArrayNew()
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORArrayLen(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORArrayLen(new RPtr(self))
            .map(Utils::boxedLongToDouble)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORArrayGet(String self, Double index, Promise promise) {
        Native.I
            .msl_bridge_cBORArrayGet(new RPtr(self), index.longValue())
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORArrayAdd(String self, String elem, Promise promise) {
        Native.I
            .msl_bridge_cBORArrayAdd(new RPtr(self), new RPtr(elem))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORArraySetDefiniteEncoding(String self, Boolean useDefinite, Promise promise) {
        Native.I
            .msl_bridge_cBORArraySetDefiniteEncoding(new RPtr(self), useDefinite)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORArrayIsDefinite(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORArrayIsDefinite(new RPtr(self))
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_cBORObjectToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORObjectToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORObjectFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cBORObjectFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORObjectNew( Promise promise) {
        Native.I
            .msl_bridge_cBORObjectNew()
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORObjectLen(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORObjectLen(new RPtr(self))
            .map(Utils::boxedLongToDouble)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORObjectInsert(String self, String key, String value, Promise promise) {
        Native.I
            .msl_bridge_cBORObjectInsert(new RPtr(self), new RPtr(key), new RPtr(value))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORObjectGet(String self, String key, Promise promise) {
        Native.I
            .msl_bridge_cBORObjectGet(new RPtr(self), new RPtr(key))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORObjectKeys(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORObjectKeys(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORObjectSetDefiniteEncoding(String self, Boolean useDefinite, Promise promise) {
        Native.I
            .msl_bridge_cBORObjectSetDefiniteEncoding(new RPtr(self), useDefinite)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORObjectIsDefinite(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORObjectIsDefinite(new RPtr(self))
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_cBORSpecialToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORSpecialToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORSpecialFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cBORSpecialFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORSpecialNewBool(Boolean b, Promise promise) {
        Native.I
            .msl_bridge_cBORSpecialNewBool(b)
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORSpecialNewUnassigned(Double u, Promise promise) {
        Native.I
            .msl_bridge_cBORSpecialNewUnassigned(u.longValue())
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORSpecialNewBreak( Promise promise) {
        Native.I
            .msl_bridge_cBORSpecialNewBreak()
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORSpecialNewNull( Promise promise) {
        Native.I
            .msl_bridge_cBORSpecialNewNull()
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORSpecialNewUndefined( Promise promise) {
        Native.I
            .msl_bridge_cBORSpecialNewUndefined()
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORSpecialKind(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORSpecialKind(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORSpecialAsBool(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORSpecialAsBool(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORSpecialAsFloat(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORSpecialAsFloat(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORSpecialAsUnassigned(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORSpecialAsUnassigned(new RPtr(self))
            .map(Utils::boxedLongToDouble)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_cBORValueToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORValueToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cBORValueFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueNewInt(String intValue, Promise promise) {
        Native.I
            .msl_bridge_cBORValueNewInt(new RPtr(intValue))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueNewBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cBORValueNewBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueNewText(String text, Promise promise) {
        Native.I
            .msl_bridge_cBORValueNewText(text)
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueNewArray(String arr, Promise promise) {
        Native.I
            .msl_bridge_cBORValueNewArray(new RPtr(arr))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueNewObject(String obj, Promise promise) {
        Native.I
            .msl_bridge_cBORValueNewObject(new RPtr(obj))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueNewTagged(String tagged, Promise promise) {
        Native.I
            .msl_bridge_cBORValueNewTagged(new RPtr(tagged))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueNewSpecial(String special, Promise promise) {
        Native.I
            .msl_bridge_cBORValueNewSpecial(new RPtr(special))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueFromLabel(String label, Promise promise) {
        Native.I
            .msl_bridge_cBORValueFromLabel(new RPtr(label))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueKind(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORValueKind(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueAsInt(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORValueAsInt(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueAsBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORValueAsBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueAsText(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORValueAsText(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueAsArray(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORValueAsArray(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueAsObject(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORValueAsObject(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueAsTagged(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORValueAsTagged(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cBORValueAsSpecial(String self, Promise promise) {
        Native.I
            .msl_bridge_cBORValueAsSpecial(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_cOSEEncryptToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSEEncryptToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEEncryptFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cOSEEncryptFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEEncryptHeaders(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSEEncryptHeaders(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEEncryptCiphertext(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSEEncryptCiphertext(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEEncryptRecipients(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSEEncryptRecipients(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEEncryptNew(String headers, String recipients, Promise promise) {
        Native.I
            .msl_bridge_cOSEEncryptNew(new RPtr(headers), new RPtr(recipients))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEEncryptNewWithCiphertext(String headers, String ciphertext, String recipients, Promise promise) {
        Native.I
            .msl_bridge_cOSEEncryptNewWithCiphertext(new RPtr(headers), Base64.decode(ciphertext, Base64.DEFAULT), new RPtr(recipients))
            .map(RPtr::toJs)
            .pour(promise);
    }



    @ReactMethod
    public final void msl_bridge_cOSEEncrypt0ToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSEEncrypt0ToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEEncrypt0FromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cOSEEncrypt0FromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEEncrypt0Headers(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSEEncrypt0Headers(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEEncrypt0Ciphertext(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSEEncrypt0Ciphertext(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEEncrypt0New(String headers, Promise promise) {
        Native.I
            .msl_bridge_cOSEEncrypt0New(new RPtr(headers))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEEncrypt0NewWithCiphertext(String headers, String ciphertext, Promise promise) {
        Native.I
            .msl_bridge_cOSEEncrypt0NewWithCiphertext(new RPtr(headers), Base64.decode(ciphertext, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }



    @ReactMethod
    public final void msl_bridge_cOSEKeyToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeyToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeyFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeyFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeySetKeyType(String self, String keyType, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeySetKeyType(new RPtr(self), new RPtr(keyType))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeyKeyType(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeyKeyType(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeySetKeyId(String self, String keyId, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeySetKeyId(new RPtr(self), Base64.decode(keyId, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeyKeyId(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeyKeyId(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeySetAlgorithmId(String self, String algorithmId, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeySetAlgorithmId(new RPtr(self), new RPtr(algorithmId))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeyAlgorithmId(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeyAlgorithmId(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeySetKeyOps(String self, String keyOps, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeySetKeyOps(new RPtr(self), new RPtr(keyOps))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeyKeyOps(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeyKeyOps(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeySetBaseInitVector(String self, String baseInitVector, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeySetBaseInitVector(new RPtr(self), Base64.decode(baseInitVector, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeyBaseInitVector(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeyBaseInitVector(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeyHeader(String self, String label, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeyHeader(new RPtr(self), new RPtr(label))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeySetHeader(String self, String label, String value, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeySetHeader(new RPtr(self), new RPtr(label), new RPtr(value))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSEKeyNew(String keyType, Promise promise) {
        Native.I
            .msl_bridge_cOSEKeyNew(new RPtr(keyType))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_cOSERecipientToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSERecipientToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSERecipientFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cOSERecipientFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSERecipientHeaders(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSERecipientHeaders(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSERecipientCiphertext(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSERecipientCiphertext(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSERecipientNew(String headers, Promise promise) {
        Native.I
            .msl_bridge_cOSERecipientNew(new RPtr(headers))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSERecipientNewWithCiphertext(String headers, String ciphertext, Promise promise) {
        Native.I
            .msl_bridge_cOSERecipientNewWithCiphertext(new RPtr(headers), Base64.decode(ciphertext, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }



    @ReactMethod
    public final void msl_bridge_cOSERecipientsToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSERecipientsToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSERecipientsFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cOSERecipientsFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSERecipientsNew( Promise promise) {
        Native.I
            .msl_bridge_cOSERecipientsNew()
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSERecipientsLen(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSERecipientsLen(new RPtr(self))
            .map(Utils::boxedLongToDouble)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSERecipientsGet(String self, Double index, Promise promise) {
        Native.I
            .msl_bridge_cOSERecipientsGet(new RPtr(self), index.longValue())
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSERecipientsAdd(String self, String elem, Promise promise) {
        Native.I
            .msl_bridge_cOSERecipientsAdd(new RPtr(self), new RPtr(elem))
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_cOSESignToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESignToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cOSESignFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignHeaders(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESignHeaders(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignPayload(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESignPayload(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignSignatures(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESignSignatures(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignNew(String headers, String signatures, Promise promise) {
        Native.I
            .msl_bridge_cOSESignNew(new RPtr(headers), new RPtr(signatures))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignNewWithPayload(String headers, String payload, String signatures, Promise promise) {
        Native.I
            .msl_bridge_cOSESignNewWithPayload(new RPtr(headers), Base64.decode(payload, Base64.DEFAULT), new RPtr(signatures))
            .map(RPtr::toJs)
            .pour(promise);
    }



    @ReactMethod
    public final void msl_bridge_cOSESign1ToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1ToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESign1FromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1FromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESign1Headers(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1Headers(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESign1Payload(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1Payload(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESign1Signature(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1Signature(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESign1SignedData(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1SignedData(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESign1SignedDataWithExternalAad(String self, String externalAad, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1SignedDataWithExternalAad(new RPtr(self), Base64.decode(externalAad, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESign1SignedDataWithExternalPayload(String self, String externalPayload, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1SignedDataWithExternalPayload(new RPtr(self), Base64.decode(externalPayload, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESign1SignedDataWithExternalAadExternalPayload(String self, String externalAad, String externalPayload, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1SignedDataWithExternalAadExternalPayload(new RPtr(self), Base64.decode(externalAad, Base64.DEFAULT), Base64.decode(externalPayload, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_cOSESign1New(String headers, String signature, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1New(new RPtr(headers), Base64.decode(signature, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESign1NewWithPayload(String headers, String payload, String signature, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1NewWithPayload(new RPtr(headers), Base64.decode(payload, Base64.DEFAULT), Base64.decode(signature, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }



    @ReactMethod
    public final void msl_bridge_cOSESign1BuilderNew(String headers, String payload, Boolean isPayloadExternal, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1BuilderNew(new RPtr(headers), Base64.decode(payload, Base64.DEFAULT), isPayloadExternal)
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESign1BuilderHashPayload(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1BuilderHashPayload(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESign1BuilderSetExternalAad(String self, String externalAad, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1BuilderSetExternalAad(new RPtr(self), Base64.decode(externalAad, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESign1BuilderMakeDataToSign(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1BuilderMakeDataToSign(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESign1BuilderBuild(String self, String signedSigStructure, Promise promise) {
        Native.I
            .msl_bridge_cOSESign1BuilderBuild(new RPtr(self), Base64.decode(signedSigStructure, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_cOSESignBuilderNew(String headers, String payload, Boolean isPayloadExternal, Promise promise) {
        Native.I
            .msl_bridge_cOSESignBuilderNew(new RPtr(headers), Base64.decode(payload, Base64.DEFAULT), isPayloadExternal)
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignBuilderHashPayload(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESignBuilderHashPayload(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignBuilderSetExternalAad(String self, String externalAad, Promise promise) {
        Native.I
            .msl_bridge_cOSESignBuilderSetExternalAad(new RPtr(self), Base64.decode(externalAad, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignBuilderMakeDataToSign(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESignBuilderMakeDataToSign(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignBuilderBuild(String self, String signedSigStructure, Promise promise) {
        Native.I
            .msl_bridge_cOSESignBuilderBuild(new RPtr(self), new RPtr(signedSigStructure))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_cOSESignatureToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESignatureToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignatureFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cOSESignatureFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignatureHeaders(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESignatureHeaders(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignatureSignature(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESignatureSignature(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignatureNew(String headers, String signature, Promise promise) {
        Native.I
            .msl_bridge_cOSESignatureNew(new RPtr(headers), Base64.decode(signature, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_cOSESignaturesToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESignaturesToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignaturesFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_cOSESignaturesFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignaturesNew( Promise promise) {
        Native.I
            .msl_bridge_cOSESignaturesNew()
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignaturesLen(String self, Promise promise) {
        Native.I
            .msl_bridge_cOSESignaturesLen(new RPtr(self))
            .map(Utils::boxedLongToDouble)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignaturesGet(String self, Double index, Promise promise) {
        Native.I
            .msl_bridge_cOSESignaturesGet(new RPtr(self), index.longValue())
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_cOSESignaturesAdd(String self, String elem, Promise promise) {
        Native.I
            .msl_bridge_cOSESignaturesAdd(new RPtr(self), new RPtr(elem))
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_counterSignatureToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_counterSignatureToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_counterSignatureFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_counterSignatureFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_counterSignatureNewSingle(String coseSignature, Promise promise) {
        Native.I
            .msl_bridge_counterSignatureNewSingle(new RPtr(coseSignature))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_counterSignatureNewMulti(String coseSignatures, Promise promise) {
        Native.I
            .msl_bridge_counterSignatureNewMulti(new RPtr(coseSignatures))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_counterSignatureSignatures(String self, Promise promise) {
        Native.I
            .msl_bridge_counterSignatureSignatures(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_edDSA25519KeyNew(String pubkeyBytes, Promise promise) {
        Native.I
            .msl_bridge_edDSA25519KeyNew(Base64.decode(pubkeyBytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_edDSA25519KeySetPrivateKey(String self, String privateKeyBytes, Promise promise) {
        Native.I
            .msl_bridge_edDSA25519KeySetPrivateKey(new RPtr(self), Base64.decode(privateKeyBytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_edDSA25519KeyIsForSigning(String self, Promise promise) {
        Native.I
            .msl_bridge_edDSA25519KeyIsForSigning(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_edDSA25519KeyIsForVerifying(String self, Promise promise) {
        Native.I
            .msl_bridge_edDSA25519KeyIsForVerifying(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_edDSA25519KeyBuild(String self, Promise promise) {
        Native.I
            .msl_bridge_edDSA25519KeyBuild(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_headerMapToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_headerMapToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_headerMapFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapSetAlgorithmId(String self, String algorithmId, Promise promise) {
        Native.I
            .msl_bridge_headerMapSetAlgorithmId(new RPtr(self), new RPtr(algorithmId))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapAlgorithmId(String self, Promise promise) {
        Native.I
            .msl_bridge_headerMapAlgorithmId(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapSetCriticality(String self, String criticality, Promise promise) {
        Native.I
            .msl_bridge_headerMapSetCriticality(new RPtr(self), new RPtr(criticality))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapCriticality(String self, Promise promise) {
        Native.I
            .msl_bridge_headerMapCriticality(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapSetContentType(String self, String contentType, Promise promise) {
        Native.I
            .msl_bridge_headerMapSetContentType(new RPtr(self), new RPtr(contentType))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapContentType(String self, Promise promise) {
        Native.I
            .msl_bridge_headerMapContentType(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapSetKeyId(String self, String keyId, Promise promise) {
        Native.I
            .msl_bridge_headerMapSetKeyId(new RPtr(self), Base64.decode(keyId, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapKeyId(String self, Promise promise) {
        Native.I
            .msl_bridge_headerMapKeyId(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapSetInitVector(String self, String initVector, Promise promise) {
        Native.I
            .msl_bridge_headerMapSetInitVector(new RPtr(self), Base64.decode(initVector, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapInitVector(String self, Promise promise) {
        Native.I
            .msl_bridge_headerMapInitVector(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapSetPartialInitVector(String self, String partialInitVector, Promise promise) {
        Native.I
            .msl_bridge_headerMapSetPartialInitVector(new RPtr(self), Base64.decode(partialInitVector, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapPartialInitVector(String self, Promise promise) {
        Native.I
            .msl_bridge_headerMapPartialInitVector(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapSetCounterSignature(String self, String counterSignature, Promise promise) {
        Native.I
            .msl_bridge_headerMapSetCounterSignature(new RPtr(self), new RPtr(counterSignature))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapCounterSignature(String self, Promise promise) {
        Native.I
            .msl_bridge_headerMapCounterSignature(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapHeader(String self, String label, Promise promise) {
        Native.I
            .msl_bridge_headerMapHeader(new RPtr(self), new RPtr(label))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapSetHeader(String self, String label, String value, Promise promise) {
        Native.I
            .msl_bridge_headerMapSetHeader(new RPtr(self), new RPtr(label), new RPtr(value))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapKeys(String self, Promise promise) {
        Native.I
            .msl_bridge_headerMapKeys(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headerMapNew( Promise promise) {
        Native.I
            .msl_bridge_headerMapNew()
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_headersToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_headersToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headersFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_headersFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headersProtected(String self, Promise promise) {
        Native.I
            .msl_bridge_headersProtected(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headersUnprotected(String self, Promise promise) {
        Native.I
            .msl_bridge_headersUnprotected(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_headersNew(String protected_, String unprotected_, Promise promise) {
        Native.I
            .msl_bridge_headersNew(new RPtr(protected_), new RPtr(unprotected_))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_intNew(String x, Promise promise) {
        Native.I
            .msl_bridge_intNew(new RPtr(x))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_intNewNegative(String x, Promise promise) {
        Native.I
            .msl_bridge_intNewNegative(new RPtr(x))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_intNewI32(Double x, Promise promise) {
        Native.I
            .msl_bridge_intNewI32(x.longValue())
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_intIsPositive(String self, Promise promise) {
        Native.I
            .msl_bridge_intIsPositive(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_intAsPositive(String self, Promise promise) {
        Native.I
            .msl_bridge_intAsPositive(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_intAsNegative(String self, Promise promise) {
        Native.I
            .msl_bridge_intAsNegative(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_intAsI32(String self, Promise promise) {
        Native.I
            .msl_bridge_intAsI32(new RPtr(self))
            .map(Utils::boxedLongToDouble)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_labelToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_labelToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_labelFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelNewInt(String intValue, Promise promise) {
        Native.I
            .msl_bridge_labelNewInt(new RPtr(intValue))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelNewText(String text, Promise promise) {
        Native.I
            .msl_bridge_labelNewText(text)
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelKind(String self, Promise promise) {
        Native.I
            .msl_bridge_labelKind(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelAsInt(String self, Promise promise) {
        Native.I
            .msl_bridge_labelAsInt(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelAsText(String self, Promise promise) {
        Native.I
            .msl_bridge_labelAsText(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelFromAlgorithmId(Double id, Promise promise) {
        Native.I
            .msl_bridge_labelFromAlgorithmId(id.intValue())
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelFromKeyType(Double keyType, Promise promise) {
        Native.I
            .msl_bridge_labelFromKeyType(keyType.intValue())
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelFromEcKey(Double ecKey, Promise promise) {
        Native.I
            .msl_bridge_labelFromEcKey(ecKey.intValue())
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelFromCurveType(Double curveType, Promise promise) {
        Native.I
            .msl_bridge_labelFromCurveType(curveType.intValue())
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelFromKeyOperation(Double keyOp, Promise promise) {
        Native.I
            .msl_bridge_labelFromKeyOperation(keyOp.intValue())
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_labelsToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_labelsToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelsFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_labelsFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelsNew( Promise promise) {
        Native.I
            .msl_bridge_labelsNew()
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelsLen(String self, Promise promise) {
        Native.I
            .msl_bridge_labelsLen(new RPtr(self))
            .map(Utils::boxedLongToDouble)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelsGet(String self, Double index, Promise promise) {
        Native.I
            .msl_bridge_labelsGet(new RPtr(self), index.longValue())
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_labelsAdd(String self, String elem, Promise promise) {
        Native.I
            .msl_bridge_labelsAdd(new RPtr(self), new RPtr(elem))
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_passwordEncryptionToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_passwordEncryptionToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_passwordEncryptionFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_passwordEncryptionFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_passwordEncryptionNew(String data, Promise promise) {
        Native.I
            .msl_bridge_passwordEncryptionNew(new RPtr(data))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_protectedHeaderMapToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_protectedHeaderMapToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_protectedHeaderMapFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_protectedHeaderMapFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_protectedHeaderMapNewEmpty( Promise promise) {
        Native.I
            .msl_bridge_protectedHeaderMapNewEmpty()
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_protectedHeaderMapNew(String headerMap, Promise promise) {
        Native.I
            .msl_bridge_protectedHeaderMapNew(new RPtr(headerMap))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_protectedHeaderMapDeserializedHeaders(String self, Promise promise) {
        Native.I
            .msl_bridge_protectedHeaderMapDeserializedHeaders(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_pubKeyEncryptionToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_pubKeyEncryptionToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_pubKeyEncryptionFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_pubKeyEncryptionFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_pubKeyEncryptionNew(String data, Promise promise) {
        Native.I
            .msl_bridge_pubKeyEncryptionNew(new RPtr(data))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_sigStructureToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_sigStructureToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_sigStructureFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_sigStructureFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_sigStructureContext(String self, Promise promise) {
        Native.I
            .msl_bridge_sigStructureContext(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_sigStructureBodyProtected(String self, Promise promise) {
        Native.I
            .msl_bridge_sigStructureBodyProtected(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_sigStructureSignProtected(String self, Promise promise) {
        Native.I
            .msl_bridge_sigStructureSignProtected(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_sigStructureExternalAad(String self, Promise promise) {
        Native.I
            .msl_bridge_sigStructureExternalAad(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_sigStructurePayload(String self, Promise promise) {
        Native.I
            .msl_bridge_sigStructurePayload(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_sigStructureSetSignProtected(String self, String signProtected, Promise promise) {
        Native.I
            .msl_bridge_sigStructureSetSignProtected(new RPtr(self), new RPtr(signProtected))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_sigStructureNew(Double context, String bodyProtected, String externalAad, String payload, Promise promise) {
        Native.I
            .msl_bridge_sigStructureNew(context.intValue(), new RPtr(bodyProtected), Base64.decode(externalAad, Base64.DEFAULT), Base64.decode(payload, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_signedMessageToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_signedMessageToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_signedMessageFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_signedMessageFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_signedMessageNewCoseSign(String coseSign, Promise promise) {
        Native.I
            .msl_bridge_signedMessageNewCoseSign(new RPtr(coseSign))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_signedMessageNewCoseSign1(String coseSign1, Promise promise) {
        Native.I
            .msl_bridge_signedMessageNewCoseSign1(new RPtr(coseSign1))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_signedMessageFromUserFacingEncoding(String s, Promise promise) {
        Native.I
            .msl_bridge_signedMessageFromUserFacingEncoding(s)
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_signedMessageToUserFacingEncoding(String self, Promise promise) {
        Native.I
            .msl_bridge_signedMessageToUserFacingEncoding(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_signedMessageKind(String self, Promise promise) {
        Native.I
            .msl_bridge_signedMessageKind(new RPtr(self))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_signedMessageAsCoseSign(String self, Promise promise) {
        Native.I
            .msl_bridge_signedMessageAsCoseSign(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_signedMessageAsCoseSign1(String self, Promise promise) {
        Native.I
            .msl_bridge_signedMessageAsCoseSign1(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }


    @ReactMethod
    public final void msl_bridge_taggedCBORToBytes(String self, Promise promise) {
        Native.I
            .msl_bridge_taggedCBORToBytes(new RPtr(self))
            .map(bytes -> Base64.encodeToString(bytes, Base64.DEFAULT))
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_taggedCBORFromBytes(String bytes, Promise promise) {
        Native.I
            .msl_bridge_taggedCBORFromBytes(Base64.decode(bytes, Base64.DEFAULT))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_taggedCBORTag(String self, Promise promise) {
        Native.I
            .msl_bridge_taggedCBORTag(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_taggedCBORValue(String self, Promise promise) {
        Native.I
            .msl_bridge_taggedCBORValue(new RPtr(self))
            .map(RPtr::toJs)
            .pour(promise);
    }

    @ReactMethod
    public final void msl_bridge_taggedCBORNew(String tag, String value, Promise promise) {
        Native.I
            .msl_bridge_taggedCBORNew(new RPtr(tag), new RPtr(value))
            .map(RPtr::toJs)
            .pour(promise);
    }


}
