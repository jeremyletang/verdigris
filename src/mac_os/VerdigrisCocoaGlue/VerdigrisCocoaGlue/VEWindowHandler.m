//
//  VEWindowHandler.m
//  VerdigrisCocoaGlue
//
//  Created by Jeremy on 07/07/2014.
//  Copyright (c) 2014 libverdigris. All rights reserved.
//

#import "VEWindowHandler.h"

@implementation VEWindowHandler

- (id) initWithWidth:(int32_t)width Height:(int32_t)height WindowStyle:(NSUInteger)style {
    // check if we are in the main thread
    if ([NSThread currentThread] != [NSThread mainThread]) {
        NSLog(@"Cannot create a new window outside the main thread.");
        return nil;
    }
    // init self
    if ((self = [super init])) {
        self->window = nil;
        
        [NSApplication sharedApplication];
        [NSApp setActivationPolicy:NSApplicationActivationPolicyRegular];
        [NSApp activateIgnoringOtherApps:YES];
        
        // Tell the application to stop bouncing in the Dock.
        [[NSApplication sharedApplication] finishLaunching];
        
        self->window = [[VEWindow alloc] initWithContentRect:NSMakeRect(0, 0, width, height)
                                                styleMask:style backing:NSBackingStoreBuffered defer:NO];
        [self->window cascadeTopLeftFromPoint:NSMakePoint(20,20)];
        [self->window setTitle:@"HELLO APP"];
        [self->window makeKeyAndOrderFront:window];
        
    }
    return self;
}

@end
