#import "MessageSigningLib.h"
#import "NSString+RPtr.h"
#import "NSData+DataPtr.h"
#import "SafeOperation.h"
#import <react_native_message_signing_library.h>


@implementation MessageSigningLib

RCT_EXPORT_MODULE()

RCT_EXPORT_METHOD(ptrFree:(NSString *)ptr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    RPtr rPtr = [ptr rPtr];
    rptr_free(&rPtr);
    resolve(nil);
}

+ (void)msl_bridge_initialize
{
    if (self == [MessageSigningLib class]) {
        init_message_signing_library();
    }
}

RCT_EXPORT_METHOD(msl_bridge_bigNumToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_big_num_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_bigNumFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_big_num_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_bigNumFromStr:(nonnull NSString *)stringVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* stringVal, CharPtr* error) {
        RPtr result;
        CharPtr string = [stringVal  charPtr];
        return msl_bridge_big_num_from_str(string, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:stringVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_bigNumToStr:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        CharPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_big_num_to_str(self, &result, error)
            ? [NSString stringFromCharPtr:&result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_bigNumCheckedMul:(nonnull NSString *)selfPtr withOther:(nonnull NSString *)otherPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr other = [[params objectAtIndex:1]  rPtr];
        return msl_bridge_big_num_checked_mul(self, other, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, otherPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_bigNumCheckedAdd:(nonnull NSString *)selfPtr withOther:(nonnull NSString *)otherPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr other = [[params objectAtIndex:1]  rPtr];
        return msl_bridge_big_num_checked_add(self, other, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, otherPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_bigNumCheckedSub:(nonnull NSString *)selfPtr withOther:(nonnull NSString *)otherPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr other = [[params objectAtIndex:1]  rPtr];
        return msl_bridge_big_num_checked_sub(self, other, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, otherPtr] andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_cBORArrayToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_array_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORArrayFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_b_o_r_array_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORArrayNew:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return msl_bridge_c_b_o_r_array_new(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORArrayLen:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        int64_t result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_array_len(self, &result, error)
            ? [NSNumber numberWithLongLong:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORArrayGet:(nonnull NSString *)selfPtr withIndex:(nonnull NSNumber *)indexVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        int64_t index = [[params objectAtIndex:1]  longLongValue];
        return msl_bridge_c_b_o_r_array_get(self, index, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, indexVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORArrayAdd:(nonnull NSString *)selfPtr withElem:(nonnull NSString *)elemPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr elem = [[params objectAtIndex:1]  rPtr];
        msl_bridge_c_b_o_r_array_add(self, elem, error);
        return nil;
    }] exec:@[selfPtr, elemPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORArraySetDefiniteEncoding:(nonnull NSString *)selfPtr withUseDefinite:(nonnull NSNumber *)useDefiniteVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        BOOL useDefinite = [[params objectAtIndex:1]  boolValue];
        msl_bridge_c_b_o_r_array_set_definite_encoding(self, useDefinite, error);
        return nil;
    }] exec:@[selfPtr, useDefiniteVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORArrayIsDefinite:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        BOOL result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_array_is_definite(self, &result, error)
            ? [NSNumber numberWithBool:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_cBORObjectToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_object_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORObjectFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_b_o_r_object_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORObjectNew:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return msl_bridge_c_b_o_r_object_new(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORObjectLen:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        int64_t result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_object_len(self, &result, error)
            ? [NSNumber numberWithLongLong:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORObjectInsert:(nonnull NSString *)selfPtr withKey:(nonnull NSString *)keyPtr withValue:(nonnull NSString *)valuePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr key = [[params objectAtIndex:1]  rPtr];
        RPtr value = [[params objectAtIndex:2]  rPtr];
        return msl_bridge_c_b_o_r_object_insert(self, key, value, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, keyPtr, valuePtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORObjectGet:(nonnull NSString *)selfPtr withKey:(nonnull NSString *)keyPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr key = [[params objectAtIndex:1]  rPtr];
        return msl_bridge_c_b_o_r_object_get(self, key, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, keyPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORObjectKeys:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_object_keys(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORObjectSetDefiniteEncoding:(nonnull NSString *)selfPtr withUseDefinite:(nonnull NSNumber *)useDefiniteVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        BOOL useDefinite = [[params objectAtIndex:1]  boolValue];
        msl_bridge_c_b_o_r_object_set_definite_encoding(self, useDefinite, error);
        return nil;
    }] exec:@[selfPtr, useDefiniteVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORObjectIsDefinite:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        BOOL result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_object_is_definite(self, &result, error)
            ? [NSNumber numberWithBool:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_cBORSpecialToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_special_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORSpecialFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_b_o_r_special_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORSpecialNewBool:(nonnull NSNumber *)bVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSNumber* bVal, CharPtr* error) {
        RPtr result;
        BOOL b = [bVal  boolValue];
        return msl_bridge_c_b_o_r_special_new_bool(b, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORSpecialNewUnassigned:(nonnull NSNumber *)uVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSNumber* uVal, CharPtr* error) {
        RPtr result;
        int64_t u = [uVal  longLongValue];
        return msl_bridge_c_b_o_r_special_new_unassigned(u, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:uVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORSpecialNewBreak:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return msl_bridge_c_b_o_r_special_new_break(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORSpecialNewNull:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return msl_bridge_c_b_o_r_special_new_null(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORSpecialNewUndefined:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return msl_bridge_c_b_o_r_special_new_undefined(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORSpecialKind:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        int32_t result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_special_kind(self, &result, error)
            ? [NSNumber numberWithLong:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORSpecialAsBool:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        BOOL result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_special_as_bool(self, &result, error)
            ? [NSNumber numberWithBool:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORSpecialAsFloat:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        int64_t result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_special_as_float(self, &result, error)
            ? [NSNumber numberWithDouble:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORSpecialAsUnassigned:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        int64_t result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_special_as_unassigned(self, &result, error)
            ? [NSNumber numberWithLongLong:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_cBORValueToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_value_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_b_o_r_value_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueNewInt:(nonnull NSString *)intValuePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* intValuePtr, CharPtr* error) {
        RPtr result;
        RPtr intValue = [intValuePtr  rPtr];
        return msl_bridge_c_b_o_r_value_new_int(intValue, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:intValuePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueNewBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_b_o_r_value_new_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueNewText:(nonnull NSString *)textVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* textVal, CharPtr* error) {
        RPtr result;
        CharPtr text = [textVal  charPtr];
        return msl_bridge_c_b_o_r_value_new_text(text, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:textVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueNewArray:(nonnull NSString *)arrPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* arrPtr, CharPtr* error) {
        RPtr result;
        RPtr arr = [arrPtr  rPtr];
        return msl_bridge_c_b_o_r_value_new_array(arr, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:arrPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueNewObject:(nonnull NSString *)objPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* objPtr, CharPtr* error) {
        RPtr result;
        RPtr obj = [objPtr  rPtr];
        return msl_bridge_c_b_o_r_value_new_object(obj, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:objPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueNewTagged:(nonnull NSString *)taggedPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* taggedPtr, CharPtr* error) {
        RPtr result;
        RPtr tagged = [taggedPtr  rPtr];
        return msl_bridge_c_b_o_r_value_new_tagged(tagged, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:taggedPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueNewSpecial:(nonnull NSString *)specialPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* specialPtr, CharPtr* error) {
        RPtr result;
        RPtr special = [specialPtr  rPtr];
        return msl_bridge_c_b_o_r_value_new_special(special, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:specialPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueFromLabel:(nonnull NSString *)labelPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* labelPtr, CharPtr* error) {
        RPtr result;
        RPtr label = [labelPtr  rPtr];
        return msl_bridge_c_b_o_r_value_from_label(label, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:labelPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueKind:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        int32_t result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_value_kind(self, &result, error)
            ? [NSNumber numberWithLong:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueAsInt:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_value_as_int(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueAsBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_value_as_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueAsText:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        CharPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_value_as_text(self, &result, error)
            ? [NSString stringFromCharPtr:&result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueAsArray:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_value_as_array(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueAsObject:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_value_as_object(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueAsTagged:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_value_as_tagged(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cBORValueAsSpecial:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_b_o_r_value_as_special(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_cOSEEncryptToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_encrypt_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEEncryptFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_o_s_e_encrypt_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEEncryptHeaders:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_encrypt_headers(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEEncryptCiphertext:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_encrypt_ciphertext(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEEncryptRecipients:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_encrypt_recipients(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEEncryptNew:(nonnull NSString *)headersPtr withRecipients:(nonnull NSString *)recipientsPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr headers = [[params objectAtIndex:0]  rPtr];
        RPtr recipients = [[params objectAtIndex:1]  rPtr];
        return msl_bridge_c_o_s_e_encrypt_new(headers, recipients, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[headersPtr, recipientsPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEEncryptNewWithCiphertext:(nonnull NSString *)headersPtr withCiphertext:(nonnull NSString *)ciphertextVal withRecipients:(nonnull NSString *)recipientsPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr headers = [[params objectAtIndex:0]  rPtr];
        NSData* dataCiphertext = [NSData fromBase64:[params objectAtIndex:1]];
        RPtr recipients = [[params objectAtIndex:2]  rPtr];
        return msl_bridge_c_o_s_e_encrypt_new_with_ciphertext(headers, (uint8_t*)dataCiphertext.bytes, dataCiphertext.length, recipients, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[headersPtr, ciphertextVal, recipientsPtr] andResolve:resolve orReject:reject];
}



RCT_EXPORT_METHOD(msl_bridge_cOSEEncrypt0ToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_encrypt0_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEEncrypt0FromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_o_s_e_encrypt0_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEEncrypt0Headers:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_encrypt0_headers(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEEncrypt0Ciphertext:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_encrypt0_ciphertext(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEEncrypt0New:(nonnull NSString *)headersPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* headersPtr, CharPtr* error) {
        RPtr result;
        RPtr headers = [headersPtr  rPtr];
        return msl_bridge_c_o_s_e_encrypt0_new(headers, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:headersPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEEncrypt0NewWithCiphertext:(nonnull NSString *)headersPtr withCiphertext:(nonnull NSString *)ciphertextVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr headers = [[params objectAtIndex:0]  rPtr];
        NSData* dataCiphertext = [NSData fromBase64:[params objectAtIndex:1]];
        return msl_bridge_c_o_s_e_encrypt0_new_with_ciphertext(headers, (uint8_t*)dataCiphertext.bytes, dataCiphertext.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[headersPtr, ciphertextVal] andResolve:resolve orReject:reject];
}



RCT_EXPORT_METHOD(msl_bridge_cOSEKeyToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_key_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeyFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_o_s_e_key_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeySetKeyType:(nonnull NSString *)selfPtr withKeyType:(nonnull NSString *)keyTypePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr keyType = [[params objectAtIndex:1]  rPtr];
        msl_bridge_c_o_s_e_key_set_key_type(self, keyType, error);
        return nil;
    }] exec:@[selfPtr, keyTypePtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeyKeyType:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_key_key_type(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeySetKeyId:(nonnull NSString *)selfPtr withKeyId:(nonnull NSString *)keyIdVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        NSData* dataKeyId = [NSData fromBase64:[params objectAtIndex:1]];
        msl_bridge_c_o_s_e_key_set_key_id(self, (uint8_t*)dataKeyId.bytes, dataKeyId.length, error);
        return nil;
    }] exec:@[selfPtr, keyIdVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeyKeyId:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_key_key_id(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeySetAlgorithmId:(nonnull NSString *)selfPtr withAlgorithmId:(nonnull NSString *)algorithmIdPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr algorithmId = [[params objectAtIndex:1]  rPtr];
        msl_bridge_c_o_s_e_key_set_algorithm_id(self, algorithmId, error);
        return nil;
    }] exec:@[selfPtr, algorithmIdPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeyAlgorithmId:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_key_algorithm_id(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeySetKeyOps:(nonnull NSString *)selfPtr withKeyOps:(nonnull NSString *)keyOpsPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr keyOps = [[params objectAtIndex:1]  rPtr];
        msl_bridge_c_o_s_e_key_set_key_ops(self, keyOps, error);
        return nil;
    }] exec:@[selfPtr, keyOpsPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeyKeyOps:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_key_key_ops(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeySetBaseInitVector:(nonnull NSString *)selfPtr withBaseInitVector:(nonnull NSString *)baseInitVectorVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        NSData* dataBaseInitVector = [NSData fromBase64:[params objectAtIndex:1]];
        msl_bridge_c_o_s_e_key_set_base_init_vector(self, (uint8_t*)dataBaseInitVector.bytes, dataBaseInitVector.length, error);
        return nil;
    }] exec:@[selfPtr, baseInitVectorVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeyBaseInitVector:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_key_base_init_vector(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeyHeader:(nonnull NSString *)selfPtr withLabel:(nonnull NSString *)labelPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr label = [[params objectAtIndex:1]  rPtr];
        return msl_bridge_c_o_s_e_key_header(self, label, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, labelPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeySetHeader:(nonnull NSString *)selfPtr withLabel:(nonnull NSString *)labelPtr withValue:(nonnull NSString *)valuePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr label = [[params objectAtIndex:1]  rPtr];
        RPtr value = [[params objectAtIndex:2]  rPtr];
        msl_bridge_c_o_s_e_key_set_header(self, label, value, error);
        return nil;
    }] exec:@[selfPtr, labelPtr, valuePtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSEKeyNew:(nonnull NSString *)keyTypePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* keyTypePtr, CharPtr* error) {
        RPtr result;
        RPtr keyType = [keyTypePtr  rPtr];
        return msl_bridge_c_o_s_e_key_new(keyType, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:keyTypePtr andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_cOSERecipientToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_recipient_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSERecipientFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_o_s_e_recipient_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSERecipientHeaders:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_recipient_headers(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSERecipientCiphertext:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_recipient_ciphertext(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSERecipientNew:(nonnull NSString *)headersPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* headersPtr, CharPtr* error) {
        RPtr result;
        RPtr headers = [headersPtr  rPtr];
        return msl_bridge_c_o_s_e_recipient_new(headers, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:headersPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSERecipientNewWithCiphertext:(nonnull NSString *)headersPtr withCiphertext:(nonnull NSString *)ciphertextVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr headers = [[params objectAtIndex:0]  rPtr];
        NSData* dataCiphertext = [NSData fromBase64:[params objectAtIndex:1]];
        return msl_bridge_c_o_s_e_recipient_new_with_ciphertext(headers, (uint8_t*)dataCiphertext.bytes, dataCiphertext.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[headersPtr, ciphertextVal] andResolve:resolve orReject:reject];
}



RCT_EXPORT_METHOD(msl_bridge_cOSERecipientsToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_recipients_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSERecipientsFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_o_s_e_recipients_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSERecipientsNew:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return msl_bridge_c_o_s_e_recipients_new(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSERecipientsLen:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        int64_t result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_recipients_len(self, &result, error)
            ? [NSNumber numberWithLongLong:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSERecipientsGet:(nonnull NSString *)selfPtr withIndex:(nonnull NSNumber *)indexVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        int64_t index = [[params objectAtIndex:1]  longLongValue];
        return msl_bridge_c_o_s_e_recipients_get(self, index, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, indexVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSERecipientsAdd:(nonnull NSString *)selfPtr withElem:(nonnull NSString *)elemPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr elem = [[params objectAtIndex:1]  rPtr];
        msl_bridge_c_o_s_e_recipients_add(self, elem, error);
        return nil;
    }] exec:@[selfPtr, elemPtr] andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_cOSESignToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_sign_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_o_s_e_sign_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignHeaders:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_sign_headers(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignPayload:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_sign_payload(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignSignatures:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_sign_signatures(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignNew:(nonnull NSString *)headersPtr withSignatures:(nonnull NSString *)signaturesPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr headers = [[params objectAtIndex:0]  rPtr];
        RPtr signatures = [[params objectAtIndex:1]  rPtr];
        return msl_bridge_c_o_s_e_sign_new(headers, signatures, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[headersPtr, signaturesPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignNewWithPayload:(nonnull NSString *)headersPtr withPayload:(nonnull NSString *)payloadVal withSignatures:(nonnull NSString *)signaturesPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr headers = [[params objectAtIndex:0]  rPtr];
        NSData* dataPayload = [NSData fromBase64:[params objectAtIndex:1]];
        RPtr signatures = [[params objectAtIndex:2]  rPtr];
        return msl_bridge_c_o_s_e_sign_new_with_payload(headers, (uint8_t*)dataPayload.bytes, dataPayload.length, signatures, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[headersPtr, payloadVal, signaturesPtr] andResolve:resolve orReject:reject];
}



RCT_EXPORT_METHOD(msl_bridge_cOSESign1ToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_sign1_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESign1FromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_o_s_e_sign1_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESign1Headers:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_sign1_headers(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESign1Payload:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_sign1_payload(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESign1Signature:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_sign1_signature(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESign1SignedData:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_sign1_signed_data(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESign1SignedDataWithExternalAad:(nonnull NSString *)selfPtr withExternalAad:(nonnull NSString *)externalAadVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        NSData* dataExternalAad = [NSData fromBase64:[params objectAtIndex:1]];
        return msl_bridge_c_o_s_e_sign1_signed_data_with_external_aad(self, (uint8_t*)dataExternalAad.bytes, dataExternalAad.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, externalAadVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESign1SignedDataWithExternalPayload:(nonnull NSString *)selfPtr withExternalPayload:(nonnull NSString *)externalPayloadVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        NSData* dataExternalPayload = [NSData fromBase64:[params objectAtIndex:1]];
        return msl_bridge_c_o_s_e_sign1_signed_data_with_external_payload(self, (uint8_t*)dataExternalPayload.bytes, dataExternalPayload.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, externalPayloadVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESign1SignedDataWithExternalAadExternalPayload:(nonnull NSString *)selfPtr withExternalAad:(nonnull NSString *)externalAadVal withExternalPayload:(nonnull NSString *)externalPayloadVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        NSData* dataExternalAad = [NSData fromBase64:[params objectAtIndex:1]];
        NSData* dataExternalPayload = [NSData fromBase64:[params objectAtIndex:2]];
        return msl_bridge_c_o_s_e_sign1_signed_data_with_external_aad_external_payload(self, (uint8_t*)dataExternalAad.bytes, dataExternalAad.length, (uint8_t*)dataExternalPayload.bytes, dataExternalPayload.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, externalAadVal, externalPayloadVal] andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_cOSESign1New:(nonnull NSString *)headersPtr withSignature:(nonnull NSString *)signatureVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr headers = [[params objectAtIndex:0]  rPtr];
        NSData* dataSignature = [NSData fromBase64:[params objectAtIndex:1]];
        return msl_bridge_c_o_s_e_sign1_new(headers, (uint8_t*)dataSignature.bytes, dataSignature.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[headersPtr, signatureVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESign1NewWithPayload:(nonnull NSString *)headersPtr withPayload:(nonnull NSString *)payloadVal withSignature:(nonnull NSString *)signatureVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr headers = [[params objectAtIndex:0]  rPtr];
        NSData* dataPayload = [NSData fromBase64:[params objectAtIndex:1]];
        NSData* dataSignature = [NSData fromBase64:[params objectAtIndex:2]];
        return msl_bridge_c_o_s_e_sign1_new_with_payload(headers, (uint8_t*)dataPayload.bytes, dataPayload.length, (uint8_t*)dataSignature.bytes, dataSignature.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[headersPtr, payloadVal, signatureVal] andResolve:resolve orReject:reject];
}



RCT_EXPORT_METHOD(msl_bridge_cOSESign1BuilderNew:(nonnull NSString *)headersPtr withPayload:(nonnull NSString *)payloadVal withIsPayloadExternal:(nonnull NSNumber *)isPayloadExternalVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr headers = [[params objectAtIndex:0]  rPtr];
        NSData* dataPayload = [NSData fromBase64:[params objectAtIndex:1]];
        BOOL isPayloadExternal = [[params objectAtIndex:2]  boolValue];
        return msl_bridge_c_o_s_e_sign1_builder_new(headers, (uint8_t*)dataPayload.bytes, dataPayload.length, isPayloadExternal, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[headersPtr, payloadVal, isPayloadExternalVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESign1BuilderHashPayload:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr self = [selfPtr  rPtr];
        msl_bridge_c_o_s_e_sign1_builder_hash_payload(self, error);
        return nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESign1BuilderSetExternalAad:(nonnull NSString *)selfPtr withExternalAad:(nonnull NSString *)externalAadVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        NSData* dataExternalAad = [NSData fromBase64:[params objectAtIndex:1]];
        msl_bridge_c_o_s_e_sign1_builder_set_external_aad(self, (uint8_t*)dataExternalAad.bytes, dataExternalAad.length, error);
        return nil;
    }] exec:@[selfPtr, externalAadVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESign1BuilderMakeDataToSign:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_sign1_builder_make_data_to_sign(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESign1BuilderBuild:(nonnull NSString *)selfPtr withSignedSigStructure:(nonnull NSString *)signedSigStructureVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        NSData* dataSignedSigStructure = [NSData fromBase64:[params objectAtIndex:1]];
        return msl_bridge_c_o_s_e_sign1_builder_build(self, (uint8_t*)dataSignedSigStructure.bytes, dataSignedSigStructure.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, signedSigStructureVal] andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_cOSESignBuilderNew:(nonnull NSString *)headersPtr withPayload:(nonnull NSString *)payloadVal withIsPayloadExternal:(nonnull NSNumber *)isPayloadExternalVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr headers = [[params objectAtIndex:0]  rPtr];
        NSData* dataPayload = [NSData fromBase64:[params objectAtIndex:1]];
        BOOL isPayloadExternal = [[params objectAtIndex:2]  boolValue];
        return msl_bridge_c_o_s_e_sign_builder_new(headers, (uint8_t*)dataPayload.bytes, dataPayload.length, isPayloadExternal, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[headersPtr, payloadVal, isPayloadExternalVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignBuilderHashPayload:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr self = [selfPtr  rPtr];
        msl_bridge_c_o_s_e_sign_builder_hash_payload(self, error);
        return nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignBuilderSetExternalAad:(nonnull NSString *)selfPtr withExternalAad:(nonnull NSString *)externalAadVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        NSData* dataExternalAad = [NSData fromBase64:[params objectAtIndex:1]];
        msl_bridge_c_o_s_e_sign_builder_set_external_aad(self, (uint8_t*)dataExternalAad.bytes, dataExternalAad.length, error);
        return nil;
    }] exec:@[selfPtr, externalAadVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignBuilderMakeDataToSign:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_sign_builder_make_data_to_sign(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignBuilderBuild:(nonnull NSString *)selfPtr withSignedSigStructure:(nonnull NSString *)signedSigStructurePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr signedSigStructure = [[params objectAtIndex:1]  rPtr];
        return msl_bridge_c_o_s_e_sign_builder_build(self, signedSigStructure, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, signedSigStructurePtr] andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_cOSESignatureToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_signature_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignatureFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_o_s_e_signature_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignatureHeaders:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_signature_headers(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignatureSignature:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_signature_signature(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignatureNew:(nonnull NSString *)headersPtr withSignature:(nonnull NSString *)signatureVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr headers = [[params objectAtIndex:0]  rPtr];
        NSData* dataSignature = [NSData fromBase64:[params objectAtIndex:1]];
        return msl_bridge_c_o_s_e_signature_new(headers, (uint8_t*)dataSignature.bytes, dataSignature.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[headersPtr, signatureVal] andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_cOSESignaturesToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_signatures_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignaturesFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_c_o_s_e_signatures_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignaturesNew:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return msl_bridge_c_o_s_e_signatures_new(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignaturesLen:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        int64_t result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_c_o_s_e_signatures_len(self, &result, error)
            ? [NSNumber numberWithLongLong:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignaturesGet:(nonnull NSString *)selfPtr withIndex:(nonnull NSNumber *)indexVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        int64_t index = [[params objectAtIndex:1]  longLongValue];
        return msl_bridge_c_o_s_e_signatures_get(self, index, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, indexVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_cOSESignaturesAdd:(nonnull NSString *)selfPtr withElem:(nonnull NSString *)elemPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr elem = [[params objectAtIndex:1]  rPtr];
        msl_bridge_c_o_s_e_signatures_add(self, elem, error);
        return nil;
    }] exec:@[selfPtr, elemPtr] andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_counterSignatureToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_counter_signature_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_counterSignatureFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_counter_signature_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_counterSignatureNewSingle:(nonnull NSString *)coseSignaturePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* coseSignaturePtr, CharPtr* error) {
        RPtr result;
        RPtr coseSignature = [coseSignaturePtr  rPtr];
        return msl_bridge_counter_signature_new_single(coseSignature, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:coseSignaturePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_counterSignatureNewMulti:(nonnull NSString *)coseSignaturesPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* coseSignaturesPtr, CharPtr* error) {
        RPtr result;
        RPtr coseSignatures = [coseSignaturesPtr  rPtr];
        return msl_bridge_counter_signature_new_multi(coseSignatures, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:coseSignaturesPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_counterSignatureSignatures:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_counter_signature_signatures(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_edDSA25519KeyNew:(nonnull NSString *)pubkeyBytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* pubkeyBytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataPubkeyBytes = [NSData fromBase64:pubkeyBytesVal];
        return msl_bridge_ed_d_s_a25519_key_new((uint8_t*)dataPubkeyBytes.bytes, dataPubkeyBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:pubkeyBytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_edDSA25519KeySetPrivateKey:(nonnull NSString *)selfPtr withPrivateKeyBytes:(nonnull NSString *)privateKeyBytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        NSData* dataPrivateKeyBytes = [NSData fromBase64:[params objectAtIndex:1]];
        msl_bridge_ed_d_s_a25519_key_set_private_key(self, (uint8_t*)dataPrivateKeyBytes.bytes, dataPrivateKeyBytes.length, error);
        return nil;
    }] exec:@[selfPtr, privateKeyBytesVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_edDSA25519KeyIsForSigning:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr self = [selfPtr  rPtr];
        msl_bridge_ed_d_s_a25519_key_is_for_signing(self, error);
        return nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_edDSA25519KeyIsForVerifying:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr self = [selfPtr  rPtr];
        msl_bridge_ed_d_s_a25519_key_is_for_verifying(self, error);
        return nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_edDSA25519KeyBuild:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_ed_d_s_a25519_key_build(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_headerMapToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_header_map_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_header_map_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapSetAlgorithmId:(nonnull NSString *)selfPtr withAlgorithmId:(nonnull NSString *)algorithmIdPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr algorithmId = [[params objectAtIndex:1]  rPtr];
        msl_bridge_header_map_set_algorithm_id(self, algorithmId, error);
        return nil;
    }] exec:@[selfPtr, algorithmIdPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapAlgorithmId:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_header_map_algorithm_id(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapSetCriticality:(nonnull NSString *)selfPtr withCriticality:(nonnull NSString *)criticalityPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr criticality = [[params objectAtIndex:1]  rPtr];
        msl_bridge_header_map_set_criticality(self, criticality, error);
        return nil;
    }] exec:@[selfPtr, criticalityPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapCriticality:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_header_map_criticality(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapSetContentType:(nonnull NSString *)selfPtr withContentType:(nonnull NSString *)contentTypePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr contentType = [[params objectAtIndex:1]  rPtr];
        msl_bridge_header_map_set_content_type(self, contentType, error);
        return nil;
    }] exec:@[selfPtr, contentTypePtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapContentType:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_header_map_content_type(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapSetKeyId:(nonnull NSString *)selfPtr withKeyId:(nonnull NSString *)keyIdVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        NSData* dataKeyId = [NSData fromBase64:[params objectAtIndex:1]];
        msl_bridge_header_map_set_key_id(self, (uint8_t*)dataKeyId.bytes, dataKeyId.length, error);
        return nil;
    }] exec:@[selfPtr, keyIdVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapKeyId:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_header_map_key_id(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapSetInitVector:(nonnull NSString *)selfPtr withInitVector:(nonnull NSString *)initVectorVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        NSData* dataInitVector = [NSData fromBase64:[params objectAtIndex:1]];
        msl_bridge_header_map_set_init_vector(self, (uint8_t*)dataInitVector.bytes, dataInitVector.length, error);
        return nil;
    }] exec:@[selfPtr, initVectorVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapInitVector:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_header_map_init_vector(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapSetPartialInitVector:(nonnull NSString *)selfPtr withPartialInitVector:(nonnull NSString *)partialInitVectorVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        NSData* dataPartialInitVector = [NSData fromBase64:[params objectAtIndex:1]];
        msl_bridge_header_map_set_partial_init_vector(self, (uint8_t*)dataPartialInitVector.bytes, dataPartialInitVector.length, error);
        return nil;
    }] exec:@[selfPtr, partialInitVectorVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapPartialInitVector:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_header_map_partial_init_vector(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapSetCounterSignature:(nonnull NSString *)selfPtr withCounterSignature:(nonnull NSString *)counterSignaturePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr counterSignature = [[params objectAtIndex:1]  rPtr];
        msl_bridge_header_map_set_counter_signature(self, counterSignature, error);
        return nil;
    }] exec:@[selfPtr, counterSignaturePtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapCounterSignature:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_header_map_counter_signature(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapHeader:(nonnull NSString *)selfPtr withLabel:(nonnull NSString *)labelPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr label = [[params objectAtIndex:1]  rPtr];
        return msl_bridge_header_map_header(self, label, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, labelPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapSetHeader:(nonnull NSString *)selfPtr withLabel:(nonnull NSString *)labelPtr withValue:(nonnull NSString *)valuePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr label = [[params objectAtIndex:1]  rPtr];
        RPtr value = [[params objectAtIndex:2]  rPtr];
        msl_bridge_header_map_set_header(self, label, value, error);
        return nil;
    }] exec:@[selfPtr, labelPtr, valuePtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapKeys:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_header_map_keys(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headerMapNew:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return msl_bridge_header_map_new(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_headersToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_headers_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headersFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_headers_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headersProtected:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_headers_protected(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headersUnprotected:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_headers_unprotected(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_headersNew:(nonnull NSString *)protected_Ptr withUnprotected_:(nonnull NSString *)unprotected_Ptr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr protected_ = [[params objectAtIndex:0]  rPtr];
        RPtr unprotected_ = [[params objectAtIndex:1]  rPtr];
        return msl_bridge_headers_new(protected_, unprotected_, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[protected_Ptr, unprotected_Ptr] andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_intNew:(nonnull NSString *)xPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* xPtr, CharPtr* error) {
        RPtr result;
        RPtr x = [xPtr  rPtr];
        return msl_bridge_int_new(x, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:xPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_intNewNegative:(nonnull NSString *)xPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* xPtr, CharPtr* error) {
        RPtr result;
        RPtr x = [xPtr  rPtr];
        return msl_bridge_int_new_negative(x, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:xPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_intNewI32:(nonnull NSNumber *)xVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSNumber* xVal, CharPtr* error) {
        RPtr result;
        int64_t x = [xVal  longLongValue];
        return msl_bridge_int_new_i32(x, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:xVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_intIsPositive:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        BOOL result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_int_is_positive(self, &result, error)
            ? [NSNumber numberWithBool:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_intAsPositive:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_int_as_positive(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_intAsNegative:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_int_as_negative(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_intAsI32:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        int64_t result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_int_as_i32(self, &result, error)
            ? [NSNumber numberWithLongLong:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_labelToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_label_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_label_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelNewInt:(nonnull NSString *)intValuePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* intValuePtr, CharPtr* error) {
        RPtr result;
        RPtr intValue = [intValuePtr  rPtr];
        return msl_bridge_label_new_int(intValue, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:intValuePtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelNewText:(nonnull NSString *)textVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* textVal, CharPtr* error) {
        RPtr result;
        CharPtr text = [textVal  charPtr];
        return msl_bridge_label_new_text(text, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:textVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelKind:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        int32_t result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_label_kind(self, &result, error)
            ? [NSNumber numberWithLong:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelAsInt:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_label_as_int(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelAsText:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        CharPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_label_as_text(self, &result, error)
            ? [NSString stringFromCharPtr:&result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelFromAlgorithmId:(nonnull NSNumber *)idVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSNumber* idVal, CharPtr* error) {
        RPtr result;
        int32_t id = [idVal  integerValue];
        return msl_bridge_label_from_algorithm_id(id, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:idVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelFromKeyType:(nonnull NSNumber *)keyTypeVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSNumber* keyTypeVal, CharPtr* error) {
        RPtr result;
        int32_t keyType = [keyTypeVal  integerValue];
        return msl_bridge_label_from_key_type(keyType, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:keyTypeVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelFromEcKey:(nonnull NSNumber *)ecKeyVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSNumber* ecKeyVal, CharPtr* error) {
        RPtr result;
        int32_t ecKey = [ecKeyVal  integerValue];
        return msl_bridge_label_from_ec_key(ecKey, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:ecKeyVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelFromCurveType:(nonnull NSNumber *)curveTypeVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSNumber* curveTypeVal, CharPtr* error) {
        RPtr result;
        int32_t curveType = [curveTypeVal  integerValue];
        return msl_bridge_label_from_curve_type(curveType, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:curveTypeVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelFromKeyOperation:(nonnull NSNumber *)keyOpVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSNumber* keyOpVal, CharPtr* error) {
        RPtr result;
        int32_t keyOp = [keyOpVal  integerValue];
        return msl_bridge_label_from_key_operation(keyOp, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:keyOpVal andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_labelsToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_labels_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelsFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_labels_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelsNew:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return msl_bridge_labels_new(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelsLen:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        int64_t result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_labels_len(self, &result, error)
            ? [NSNumber numberWithLongLong:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelsGet:(nonnull NSString *)selfPtr withIndex:(nonnull NSNumber *)indexVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr self = [[params objectAtIndex:0]  rPtr];
        int64_t index = [[params objectAtIndex:1]  longLongValue];
        return msl_bridge_labels_get(self, index, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[selfPtr, indexVal] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_labelsAdd:(nonnull NSString *)selfPtr withElem:(nonnull NSString *)elemPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr elem = [[params objectAtIndex:1]  rPtr];
        msl_bridge_labels_add(self, elem, error);
        return nil;
    }] exec:@[selfPtr, elemPtr] andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_passwordEncryptionToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_password_encryption_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_passwordEncryptionFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_password_encryption_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_passwordEncryptionNew:(nonnull NSString *)dataPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* dataPtr, CharPtr* error) {
        RPtr result;
        RPtr data = [dataPtr  rPtr];
        return msl_bridge_password_encryption_new(data, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:dataPtr andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_protectedHeaderMapToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_protected_header_map_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_protectedHeaderMapFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_protected_header_map_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_protectedHeaderMapNewEmpty:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(id _void, CharPtr* error) {
        RPtr result;
        return msl_bridge_protected_header_map_new_empty(&result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:nil andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_protectedHeaderMapNew:(nonnull NSString *)headerMapPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* headerMapPtr, CharPtr* error) {
        RPtr result;
        RPtr headerMap = [headerMapPtr  rPtr];
        return msl_bridge_protected_header_map_new(headerMap, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:headerMapPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_protectedHeaderMapDeserializedHeaders:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_protected_header_map_deserialized_headers(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_pubKeyEncryptionToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_pub_key_encryption_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_pubKeyEncryptionFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_pub_key_encryption_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_pubKeyEncryptionNew:(nonnull NSString *)dataPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* dataPtr, CharPtr* error) {
        RPtr result;
        RPtr data = [dataPtr  rPtr];
        return msl_bridge_pub_key_encryption_new(data, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:dataPtr andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_sigStructureToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_sig_structure_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_sigStructureFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_sig_structure_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_sigStructureContext:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        int32_t result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_sig_structure_context(self, &result, error)
            ? [NSNumber numberWithLong:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_sigStructureBodyProtected:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_sig_structure_body_protected(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_sigStructureSignProtected:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_sig_structure_sign_protected(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_sigStructureExternalAad:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_sig_structure_external_aad(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_sigStructurePayload:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_sig_structure_payload(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_sigStructureSetSignProtected:(nonnull NSString *)selfPtr withSignProtected:(nonnull NSString *)signProtectedPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr self = [[params objectAtIndex:0]  rPtr];
        RPtr signProtected = [[params objectAtIndex:1]  rPtr];
        msl_bridge_sig_structure_set_sign_protected(self, signProtected, error);
        return nil;
    }] exec:@[selfPtr, signProtectedPtr] andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_sigStructureNew:(nonnull NSNumber *)contextVal withBodyProtected:(nonnull NSString *)bodyProtectedPtr withExternalAad:(nonnull NSString *)externalAadVal withPayload:(nonnull NSString *)payloadVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        int32_t context = [[params objectAtIndex:0]  integerValue];
        RPtr bodyProtected = [[params objectAtIndex:1]  rPtr];
        NSData* dataExternalAad = [NSData fromBase64:[params objectAtIndex:2]];
        NSData* dataPayload = [NSData fromBase64:[params objectAtIndex:3]];
        return msl_bridge_sig_structure_new(context, bodyProtected, (uint8_t*)dataExternalAad.bytes, dataExternalAad.length, (uint8_t*)dataPayload.bytes, dataPayload.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[contextVal, bodyProtectedPtr, externalAadVal, payloadVal] andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_signedMessageToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_signed_message_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_signedMessageFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_signed_message_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_signedMessageNewCoseSign:(nonnull NSString *)coseSignPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* coseSignPtr, CharPtr* error) {
        RPtr result;
        RPtr coseSign = [coseSignPtr  rPtr];
        return msl_bridge_signed_message_new_cose_sign(coseSign, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:coseSignPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_signedMessageNewCoseSign1:(nonnull NSString *)coseSign1Ptr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* coseSign1Ptr, CharPtr* error) {
        RPtr result;
        RPtr coseSign1 = [coseSign1Ptr  rPtr];
        return msl_bridge_signed_message_new_cose_sign1(coseSign1, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:coseSign1Ptr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_signedMessageFromUserFacingEncoding:(nonnull NSString *)sVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* sVal, CharPtr* error) {
        RPtr result;
        CharPtr s = [sVal  charPtr];
        return msl_bridge_signed_message_from_user_facing_encoding(s, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:sVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_signedMessageToUserFacingEncoding:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        CharPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_signed_message_to_user_facing_encoding(self, &result, error)
            ? [NSString stringFromCharPtr:&result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_signedMessageKind:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSNumber*(NSString* selfPtr, CharPtr* error) {
        int32_t result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_signed_message_kind(self, &result, error)
            ? [NSNumber numberWithLong:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_signedMessageAsCoseSign:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_signed_message_as_cose_sign(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_signedMessageAsCoseSign1:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_signed_message_as_cose_sign1(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}


RCT_EXPORT_METHOD(msl_bridge_taggedCBORToBytes:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        DataPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_tagged_c_b_o_r_to_bytes(self, &result, error)
            ? [[NSData fromDataPtr:&result] base64]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_taggedCBORFromBytes:(nonnull NSString *)bytesVal withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* bytesVal, CharPtr* error) {
        RPtr result;
        NSData* dataBytes = [NSData fromBase64:bytesVal];
        return msl_bridge_tagged_c_b_o_r_from_bytes((uint8_t*)dataBytes.bytes, dataBytes.length, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:bytesVal andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_taggedCBORTag:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_tagged_c_b_o_r_tag(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_taggedCBORValue:(nonnull NSString *)selfPtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSString* selfPtr, CharPtr* error) {
        RPtr result;
        RPtr self = [selfPtr  rPtr];
        return msl_bridge_tagged_c_b_o_r_value(self, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:selfPtr andResolve:resolve orReject:reject];
}

RCT_EXPORT_METHOD(msl_bridge_taggedCBORNew:(nonnull NSString *)tagPtr withValue:(nonnull NSString *)valuePtr withResolve:(RCTPromiseResolveBlock)resolve andReject:(RCTPromiseRejectBlock)reject)
{
    [[MSLCSafeOperation new:^NSString*(NSArray* params, CharPtr* error) {
        RPtr result;
        RPtr tag = [[params objectAtIndex:0]  rPtr];
        RPtr value = [[params objectAtIndex:1]  rPtr];
        return msl_bridge_tagged_c_b_o_r_new(tag, value, &result, error)
            ? [NSString stringFromPtr:result]
            : nil;
    }] exec:@[tagPtr, valuePtr] andResolve:resolve orReject:reject];
}


@end
