//
//  MSLSafeOperation.h
//

#import <Foundation/Foundation.h>
#import <React/RCTBridgeModule.h>
#import <react_native_message_signing_library.h>

NS_ASSUME_NONNULL_BEGIN

//#define CHECK_NON_NULL_OR_CERROR(var, error, name) if (var == NULL) {\
//    error = copy_string([[NSString stringWithFormat:@"Empty parameter: \"%s\"", name] UTF8String]);\
//    return nil;\
//}
//
//#define CHECK_HAS_LENGTH_OR_CERROR(var, error, name) if (var == NULL || [var length] <= 0) {\
//    error = copy_string([[NSString stringWithFormat:@"Empty parameter: \"%s\"", name] UTF8String]);\
//    return nil;\
//}

@interface NSError (Rust)

+ (NSError *)rustError:(NSString *)description;

@end

@interface MSLBaseSafeOperation<In, Out> : NSObject

- (Out)exec:(In)param error:(NSError **)error;

- (void)exec:(_Nullable In)param andResolve:(RCTPromiseResolveBlock)resolve orReject:(RCTPromiseRejectBlock)reject;

@end

@interface MSLSafeOperation<In, Out> : MSLBaseSafeOperation<In, Out>

+ (MSLBaseSafeOperation<In, Out> *)new:(Out(^)(In param, NSError** error))cb;

- (MSLSafeOperation<In, Out> *)initWithCallback:(Out(^)(In param, NSError** error))cb;

@end

@interface MSLCSafeOperation<In, Out> : MSLSafeOperation<In, Out>

+ (MSLBaseSafeOperation *)new:(Out(^)(In param, CharPtr _Nullable* _Nonnull error))cb;

- (MSLCSafeOperation *)initWithCallback:(Out(^)(In param, CharPtr _Nullable* _Nonnull error))cb;

@end

@interface MSLSafeOperationCombined<In1, Out1, Out2> : MSLBaseSafeOperation<In1, Out2>

+ (MSLBaseSafeOperation<In1, Out2>* )combine:(MSLBaseSafeOperation<In1, Out1> *)op1
                                    with:(MSLBaseSafeOperation<Out1, Out2> *)op2;

- (MSLSafeOperationCombined<In1, Out1, Out2> *)init:(MSLBaseSafeOperation<In1, Out1> *)op1
                                                and:(MSLBaseSafeOperation<Out1, Out2> *)op2;

@end

NS_ASSUME_NONNULL_END
