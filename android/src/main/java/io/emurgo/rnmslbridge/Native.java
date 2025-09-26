package io.emurgo.rnmslbridge;
import java.util.Map;

final class Native {
    static final Native I;

    static {
        I = new Native();
        System.loadLibrary("react_native_message_signing_library");
        I.msl_bridge_initLibrary();
    }

    private Native() { } 
    private native void msl_bridge_initLibrary();

    public final native void ptrFree(RPtr ptr);

    public final native Result<byte[]> msl_bridge_bigNumToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_bigNumFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_bigNumFromStr(String string);
    public final native Result<String> msl_bridge_bigNumToStr(RPtr self);
    public final native Result<RPtr> msl_bridge_bigNumCheckedMul(RPtr self, RPtr other);
    public final native Result<RPtr> msl_bridge_bigNumCheckedAdd(RPtr self, RPtr other);
    public final native Result<RPtr> msl_bridge_bigNumCheckedSub(RPtr self, RPtr other);

    public final native Result<byte[]> msl_bridge_cBORArrayToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_cBORArrayFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_cBORArrayNew();
    public final native Result<Long> msl_bridge_cBORArrayLen(RPtr self);
    public final native Result<RPtr> msl_bridge_cBORArrayGet(RPtr self, long index);
    public final native Result<Void> msl_bridge_cBORArrayAdd(RPtr self, RPtr elem);
    public final native Result<Void> msl_bridge_cBORArraySetDefiniteEncoding(RPtr self, boolean useDefinite);
    public final native Result<Boolean> msl_bridge_cBORArrayIsDefinite(RPtr self);

    public final native Result<byte[]> msl_bridge_cBORObjectToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_cBORObjectFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_cBORObjectNew();
    public final native Result<Long> msl_bridge_cBORObjectLen(RPtr self);
    public final native Result<RPtr> msl_bridge_cBORObjectInsert(RPtr self, RPtr key, RPtr value);
    public final native Result<RPtr> msl_bridge_cBORObjectGet(RPtr self, RPtr key);
    public final native Result<RPtr> msl_bridge_cBORObjectKeys(RPtr self);
    public final native Result<Void> msl_bridge_cBORObjectSetDefiniteEncoding(RPtr self, boolean useDefinite);
    public final native Result<Boolean> msl_bridge_cBORObjectIsDefinite(RPtr self);

    public final native Result<byte[]> msl_bridge_cBORSpecialToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_cBORSpecialFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_cBORSpecialNewBool(boolean b);
    public final native Result<RPtr> msl_bridge_cBORSpecialNewUnassigned(long u);
    public final native Result<RPtr> msl_bridge_cBORSpecialNewBreak();
    public final native Result<RPtr> msl_bridge_cBORSpecialNewNull();
    public final native Result<RPtr> msl_bridge_cBORSpecialNewUndefined();
    public final native Result<Integer> msl_bridge_cBORSpecialKind(RPtr self);
    public final native Result<Boolean> msl_bridge_cBORSpecialAsBool(RPtr self);
    public final native Result<Double> msl_bridge_cBORSpecialAsFloat(RPtr self);
    public final native Result<Long> msl_bridge_cBORSpecialAsUnassigned(RPtr self);

    public final native Result<byte[]> msl_bridge_cBORValueToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_cBORValueFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_cBORValueNewInt(RPtr intValue);
    public final native Result<RPtr> msl_bridge_cBORValueNewBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_cBORValueNewText(String text);
    public final native Result<RPtr> msl_bridge_cBORValueNewArray(RPtr arr);
    public final native Result<RPtr> msl_bridge_cBORValueNewObject(RPtr obj);
    public final native Result<RPtr> msl_bridge_cBORValueNewTagged(RPtr tagged);
    public final native Result<RPtr> msl_bridge_cBORValueNewSpecial(RPtr special);
    public final native Result<RPtr> msl_bridge_cBORValueFromLabel(RPtr label);
    public final native Result<Integer> msl_bridge_cBORValueKind(RPtr self);
    public final native Result<RPtr> msl_bridge_cBORValueAsInt(RPtr self);
    public final native Result<byte[]> msl_bridge_cBORValueAsBytes(RPtr self);
    public final native Result<String> msl_bridge_cBORValueAsText(RPtr self);
    public final native Result<RPtr> msl_bridge_cBORValueAsArray(RPtr self);
    public final native Result<RPtr> msl_bridge_cBORValueAsObject(RPtr self);
    public final native Result<RPtr> msl_bridge_cBORValueAsTagged(RPtr self);
    public final native Result<RPtr> msl_bridge_cBORValueAsSpecial(RPtr self);

    public final native Result<byte[]> msl_bridge_cOSEEncryptToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSEEncryptFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_cOSEEncryptHeaders(RPtr self);
    public final native Result<byte[]> msl_bridge_cOSEEncryptCiphertext(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSEEncryptRecipients(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSEEncryptNew(RPtr headers, RPtr recipients);
    public final native Result<RPtr> msl_bridge_cOSEEncryptNewWithCiphertext(RPtr headers, byte[] ciphertext, RPtr recipients);


    public final native Result<byte[]> msl_bridge_cOSEEncrypt0ToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSEEncrypt0FromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_cOSEEncrypt0Headers(RPtr self);
    public final native Result<byte[]> msl_bridge_cOSEEncrypt0Ciphertext(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSEEncrypt0New(RPtr headers);
    public final native Result<RPtr> msl_bridge_cOSEEncrypt0NewWithCiphertext(RPtr headers, byte[] ciphertext);


    public final native Result<byte[]> msl_bridge_cOSEKeyToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSEKeyFromBytes(byte[] bytes);
    public final native Result<Void> msl_bridge_cOSEKeySetKeyType(RPtr self, RPtr keyType);
    public final native Result<RPtr> msl_bridge_cOSEKeyKeyType(RPtr self);
    public final native Result<Void> msl_bridge_cOSEKeySetKeyId(RPtr self, byte[] keyId);
    public final native Result<byte[]> msl_bridge_cOSEKeyKeyId(RPtr self);
    public final native Result<Void> msl_bridge_cOSEKeySetAlgorithmId(RPtr self, RPtr algorithmId);
    public final native Result<RPtr> msl_bridge_cOSEKeyAlgorithmId(RPtr self);
    public final native Result<Void> msl_bridge_cOSEKeySetKeyOps(RPtr self, RPtr keyOps);
    public final native Result<RPtr> msl_bridge_cOSEKeyKeyOps(RPtr self);
    public final native Result<Void> msl_bridge_cOSEKeySetBaseInitVector(RPtr self, byte[] baseInitVector);
    public final native Result<byte[]> msl_bridge_cOSEKeyBaseInitVector(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSEKeyHeader(RPtr self, RPtr label);
    public final native Result<Void> msl_bridge_cOSEKeySetHeader(RPtr self, RPtr label, RPtr value);
    public final native Result<RPtr> msl_bridge_cOSEKeyNew(RPtr keyType);

    public final native Result<byte[]> msl_bridge_cOSERecipientToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSERecipientFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_cOSERecipientHeaders(RPtr self);
    public final native Result<byte[]> msl_bridge_cOSERecipientCiphertext(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSERecipientNew(RPtr headers);
    public final native Result<RPtr> msl_bridge_cOSERecipientNewWithCiphertext(RPtr headers, byte[] ciphertext);


    public final native Result<byte[]> msl_bridge_cOSERecipientsToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSERecipientsFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_cOSERecipientsNew();
    public final native Result<Long> msl_bridge_cOSERecipientsLen(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSERecipientsGet(RPtr self, long index);
    public final native Result<Void> msl_bridge_cOSERecipientsAdd(RPtr self, RPtr elem);

    public final native Result<byte[]> msl_bridge_cOSESignToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSESignFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_cOSESignHeaders(RPtr self);
    public final native Result<byte[]> msl_bridge_cOSESignPayload(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSESignSignatures(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSESignNew(RPtr headers, RPtr signatures);
    public final native Result<RPtr> msl_bridge_cOSESignNewWithPayload(RPtr headers, byte[] payload, RPtr signatures);


    public final native Result<byte[]> msl_bridge_cOSESign1ToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSESign1FromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_cOSESign1Headers(RPtr self);
    public final native Result<byte[]> msl_bridge_cOSESign1Payload(RPtr self);
    public final native Result<byte[]> msl_bridge_cOSESign1Signature(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSESign1SignedData(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSESign1SignedDataWithExternalAad(RPtr self, byte[] externalAad);
    public final native Result<RPtr> msl_bridge_cOSESign1SignedDataWithExternalPayload(RPtr self, byte[] externalPayload);
    public final native Result<RPtr> msl_bridge_cOSESign1SignedDataWithExternalAadExternalPayload(RPtr self, byte[] externalAad, byte[] externalPayload);

    public final native Result<RPtr> msl_bridge_cOSESign1New(RPtr headers, byte[] signature);
    public final native Result<RPtr> msl_bridge_cOSESign1NewWithPayload(RPtr headers, byte[] payload, byte[] signature);


    public final native Result<RPtr> msl_bridge_cOSESign1BuilderNew(RPtr headers, byte[] payload, boolean isPayloadExternal);
    public final native Result<Void> msl_bridge_cOSESign1BuilderHashPayload(RPtr self);
    public final native Result<Void> msl_bridge_cOSESign1BuilderSetExternalAad(RPtr self, byte[] externalAad);
    public final native Result<RPtr> msl_bridge_cOSESign1BuilderMakeDataToSign(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSESign1BuilderBuild(RPtr self, byte[] signedSigStructure);

    public final native Result<RPtr> msl_bridge_cOSESignBuilderNew(RPtr headers, byte[] payload, boolean isPayloadExternal);
    public final native Result<Void> msl_bridge_cOSESignBuilderHashPayload(RPtr self);
    public final native Result<Void> msl_bridge_cOSESignBuilderSetExternalAad(RPtr self, byte[] externalAad);
    public final native Result<RPtr> msl_bridge_cOSESignBuilderMakeDataToSign(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSESignBuilderBuild(RPtr self, RPtr signedSigStructure);

    public final native Result<byte[]> msl_bridge_cOSESignatureToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSESignatureFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_cOSESignatureHeaders(RPtr self);
    public final native Result<byte[]> msl_bridge_cOSESignatureSignature(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSESignatureNew(RPtr headers, byte[] signature);

    public final native Result<byte[]> msl_bridge_cOSESignaturesToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSESignaturesFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_cOSESignaturesNew();
    public final native Result<Long> msl_bridge_cOSESignaturesLen(RPtr self);
    public final native Result<RPtr> msl_bridge_cOSESignaturesGet(RPtr self, long index);
    public final native Result<Void> msl_bridge_cOSESignaturesAdd(RPtr self, RPtr elem);

    public final native Result<byte[]> msl_bridge_counterSignatureToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_counterSignatureFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_counterSignatureNewSingle(RPtr coseSignature);
    public final native Result<RPtr> msl_bridge_counterSignatureNewMulti(RPtr coseSignatures);
    public final native Result<RPtr> msl_bridge_counterSignatureSignatures(RPtr self);

    public final native Result<RPtr> msl_bridge_edDSA25519KeyNew(byte[] pubkeyBytes);
    public final native Result<Void> msl_bridge_edDSA25519KeySetPrivateKey(RPtr self, byte[] privateKeyBytes);
    public final native Result<Void> msl_bridge_edDSA25519KeyIsForSigning(RPtr self);
    public final native Result<Void> msl_bridge_edDSA25519KeyIsForVerifying(RPtr self);
    public final native Result<RPtr> msl_bridge_edDSA25519KeyBuild(RPtr self);

    public final native Result<byte[]> msl_bridge_headerMapToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_headerMapFromBytes(byte[] bytes);
    public final native Result<Void> msl_bridge_headerMapSetAlgorithmId(RPtr self, RPtr algorithmId);
    public final native Result<RPtr> msl_bridge_headerMapAlgorithmId(RPtr self);
    public final native Result<Void> msl_bridge_headerMapSetCriticality(RPtr self, RPtr criticality);
    public final native Result<RPtr> msl_bridge_headerMapCriticality(RPtr self);
    public final native Result<Void> msl_bridge_headerMapSetContentType(RPtr self, RPtr contentType);
    public final native Result<RPtr> msl_bridge_headerMapContentType(RPtr self);
    public final native Result<Void> msl_bridge_headerMapSetKeyId(RPtr self, byte[] keyId);
    public final native Result<byte[]> msl_bridge_headerMapKeyId(RPtr self);
    public final native Result<Void> msl_bridge_headerMapSetInitVector(RPtr self, byte[] initVector);
    public final native Result<byte[]> msl_bridge_headerMapInitVector(RPtr self);
    public final native Result<Void> msl_bridge_headerMapSetPartialInitVector(RPtr self, byte[] partialInitVector);
    public final native Result<byte[]> msl_bridge_headerMapPartialInitVector(RPtr self);
    public final native Result<Void> msl_bridge_headerMapSetCounterSignature(RPtr self, RPtr counterSignature);
    public final native Result<RPtr> msl_bridge_headerMapCounterSignature(RPtr self);
    public final native Result<RPtr> msl_bridge_headerMapHeader(RPtr self, RPtr label);
    public final native Result<Void> msl_bridge_headerMapSetHeader(RPtr self, RPtr label, RPtr value);
    public final native Result<RPtr> msl_bridge_headerMapKeys(RPtr self);
    public final native Result<RPtr> msl_bridge_headerMapNew();

    public final native Result<byte[]> msl_bridge_headersToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_headersFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_headersProtected(RPtr self);
    public final native Result<RPtr> msl_bridge_headersUnprotected(RPtr self);
    public final native Result<RPtr> msl_bridge_headersNew(RPtr protected_, RPtr unprotected_);

    public final native Result<RPtr> msl_bridge_intNew(RPtr x);
    public final native Result<RPtr> msl_bridge_intNewNegative(RPtr x);
    public final native Result<RPtr> msl_bridge_intNewI32(long x);
    public final native Result<Boolean> msl_bridge_intIsPositive(RPtr self);
    public final native Result<RPtr> msl_bridge_intAsPositive(RPtr self);
    public final native Result<RPtr> msl_bridge_intAsNegative(RPtr self);
    public final native Result<Long> msl_bridge_intAsI32(RPtr self);

    public final native Result<byte[]> msl_bridge_labelToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_labelFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_labelNewInt(RPtr intValue);
    public final native Result<RPtr> msl_bridge_labelNewText(String text);
    public final native Result<Integer> msl_bridge_labelKind(RPtr self);
    public final native Result<RPtr> msl_bridge_labelAsInt(RPtr self);
    public final native Result<String> msl_bridge_labelAsText(RPtr self);
    public final native Result<RPtr> msl_bridge_labelFromAlgorithmId(int id);
    public final native Result<RPtr> msl_bridge_labelFromKeyType(int keyType);
    public final native Result<RPtr> msl_bridge_labelFromEcKey(int ecKey);
    public final native Result<RPtr> msl_bridge_labelFromCurveType(int curveType);
    public final native Result<RPtr> msl_bridge_labelFromKeyOperation(int keyOp);

    public final native Result<byte[]> msl_bridge_labelsToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_labelsFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_labelsNew();
    public final native Result<Long> msl_bridge_labelsLen(RPtr self);
    public final native Result<RPtr> msl_bridge_labelsGet(RPtr self, long index);
    public final native Result<Void> msl_bridge_labelsAdd(RPtr self, RPtr elem);

    public final native Result<byte[]> msl_bridge_passwordEncryptionToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_passwordEncryptionFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_passwordEncryptionNew(RPtr data);

    public final native Result<byte[]> msl_bridge_protectedHeaderMapToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_protectedHeaderMapFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_protectedHeaderMapNewEmpty();
    public final native Result<RPtr> msl_bridge_protectedHeaderMapNew(RPtr headerMap);
    public final native Result<RPtr> msl_bridge_protectedHeaderMapDeserializedHeaders(RPtr self);

    public final native Result<byte[]> msl_bridge_pubKeyEncryptionToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_pubKeyEncryptionFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_pubKeyEncryptionNew(RPtr data);

    public final native Result<byte[]> msl_bridge_sigStructureToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_sigStructureFromBytes(byte[] bytes);
    public final native Result<Integer> msl_bridge_sigStructureContext(RPtr self);
    public final native Result<RPtr> msl_bridge_sigStructureBodyProtected(RPtr self);
    public final native Result<RPtr> msl_bridge_sigStructureSignProtected(RPtr self);
    public final native Result<byte[]> msl_bridge_sigStructureExternalAad(RPtr self);
    public final native Result<byte[]> msl_bridge_sigStructurePayload(RPtr self);
    public final native Result<Void> msl_bridge_sigStructureSetSignProtected(RPtr self, RPtr signProtected);
    public final native Result<RPtr> msl_bridge_sigStructureNew(int context, RPtr bodyProtected, byte[] externalAad, byte[] payload);

    public final native Result<byte[]> msl_bridge_signedMessageToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_signedMessageFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_signedMessageNewCoseSign(RPtr coseSign);
    public final native Result<RPtr> msl_bridge_signedMessageNewCoseSign1(RPtr coseSign1);
    public final native Result<RPtr> msl_bridge_signedMessageFromUserFacingEncoding(String s);
    public final native Result<String> msl_bridge_signedMessageToUserFacingEncoding(RPtr self);
    public final native Result<Integer> msl_bridge_signedMessageKind(RPtr self);
    public final native Result<RPtr> msl_bridge_signedMessageAsCoseSign(RPtr self);
    public final native Result<RPtr> msl_bridge_signedMessageAsCoseSign1(RPtr self);

    public final native Result<byte[]> msl_bridge_taggedCBORToBytes(RPtr self);
    public final native Result<RPtr> msl_bridge_taggedCBORFromBytes(byte[] bytes);
    public final native Result<RPtr> msl_bridge_taggedCBORTag(RPtr self);
    public final native Result<RPtr> msl_bridge_taggedCBORValue(RPtr self);
    public final native Result<RPtr> msl_bridge_taggedCBORNew(RPtr tag, RPtr value);

}
