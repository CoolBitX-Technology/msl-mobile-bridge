use crate::ptr::RPtrRepresentable;
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
use cardano_message_signing::Labels;
use cardano_message_signing::PasswordEncryption;
use cardano_message_signing::ProtectedHeaderMap;
use cardano_message_signing::PubKeyEncryption;
use cardano_message_signing::SigStructure;
use cardano_message_signing::SignedMessage;
use cardano_message_signing::builders::COSESign1Builder;
use cardano_message_signing::builders::COSESignBuilder;
use cardano_message_signing::builders::EdDSA25519Key;
use cardano_message_signing::cbor::CBORArray;
use cardano_message_signing::cbor::CBORObject;
use cardano_message_signing::cbor::CBORSpecial;
use cardano_message_signing::cbor::CBORValue;
use cardano_message_signing::cbor::TaggedCBOR;
use cardano_message_signing::utils::BigNum;
use cardano_message_signing::utils::Int;
impl RPtrRepresentable for BigNum {}
impl RPtrRepresentable for CBORArray {}
impl RPtrRepresentable for CBORObject {}
impl RPtrRepresentable for CBORSpecial {}
impl RPtrRepresentable for CBORValue {}
impl RPtrRepresentable for COSEEncrypt {}
impl RPtrRepresentable for COSEEncrypt0 {}
impl RPtrRepresentable for COSEKey {}
impl RPtrRepresentable for COSERecipient {}
impl RPtrRepresentable for COSERecipients {}
impl RPtrRepresentable for COSESign {}
impl RPtrRepresentable for COSESign1 {}
impl RPtrRepresentable for COSESign1Builder {}
impl RPtrRepresentable for COSESignBuilder {}
impl RPtrRepresentable for COSESignature {}
impl RPtrRepresentable for COSESignatures {}
impl RPtrRepresentable for CounterSignature {}
impl RPtrRepresentable for EdDSA25519Key {}
impl RPtrRepresentable for HeaderMap {}
impl RPtrRepresentable for Headers {}
impl RPtrRepresentable for Int {}
impl RPtrRepresentable for Label {}
impl RPtrRepresentable for Labels {}
impl RPtrRepresentable for PasswordEncryption {}
impl RPtrRepresentable for ProtectedHeaderMap {}
impl RPtrRepresentable for PubKeyEncryption {}
impl RPtrRepresentable for SigStructure {}
impl RPtrRepresentable for SignedMessage {}
impl RPtrRepresentable for TaggedCBOR {}
