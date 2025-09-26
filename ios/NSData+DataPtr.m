//
//  NSData+DataPtr.m
//
//  Created by Ostap Danylovych on 24.10.2019.
//  Copyright © 2019 Facebook. All rights reserved.
//

#import "NSData+DataPtr.h"

@implementation NSData (DataPtr)

+ (NSData *)fromDataPtr:(DataPtr *)ptr {
    NSData* data = [NSData dataWithBytes:ptr->ptr length:ptr->len];
    msl_dataptr_free(ptr);
    return data;
}

+ (NSData *)fromBase64:(NSString *)base64Encoded {
    return [[NSData alloc] initWithBase64EncodedString:base64Encoded options:0];
}

- (NSString *)base64 {
    return [self base64EncodedStringWithOptions:0];
}

@end
