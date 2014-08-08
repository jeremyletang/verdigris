//
//  VeGL.m
//  libverdigrisglue
//
//  Created by Jeremy on 08/08/2014.
//  Copyright (c) 2014 libverdigris. All rights reserved.
//

#import "ve_gl.h"
#import <mach-o/dyld.h>

void *ve_get_proc_address(const char *proc_name) {
    void *framework = CFBundleGetBundleWithIdentifier(CFSTR("com.apple.opengl"));
    void *symbol = NULL;
    
    if (framework != NULL) {
        CFStringRef symbol_name = CFStringCreateWithCString(kCFAllocatorDefault,
                                                           proc_name,
                                                           kCFStringEncodingASCII);
    
        symbol = CFBundleGetFunctionPointerForName(framework,
                                                   symbol_name);
    
        CFRelease(symbol_name);
    }
    
    return symbol;
}
