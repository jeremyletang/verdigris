//
//  VEWindowHandler.m
//  libverdigrisglue
//
//  Created by Jeremy on 07/07/2014.
//  Copyright (c) 2014 libverdigris. All rights reserved.
//

#import "ve_window_handler.h"

@implementation VEWindowHandler

- (id) initWithSize:(NSSize)size AndWindowStyle:(NSUInteger)style {
    // check if we are in the main thread
    if ([NSThread currentThread] != [NSThread mainThread]) {
        NSLog(@"Cannot create a new window outside the main thread.");
        return nil;
    }

    // init self super
    if ((self = [super init])) {
        self->window = nil;
        self->shouldClose = NO;

        [NSApplication sharedApplication];
        [NSApp setActivationPolicy: NSApplicationActivationPolicyRegular];
        [NSApp activateIgnoringOtherApps: YES];

        // should stop bounce.
        [[NSApplication sharedApplication] finishLaunching];

        self->window = [[VEWindow alloc] initWithContentRect: NSMakeRect(0, 0, size.width, size.height)
                                                   styleMask: style
                                                     backing: NSBackingStoreBuffered
                                                       defer: NO];


        //        // Create the view.
        //        self->glView = [[NSOpenGLView alloc] initWithFrame:[[self->window contentView] frame]];
        //
        //        if (self->glView == nil) {
        //            NSLog(@"Could not create an instance of NSOpenGLView ");
        //            return nil;
        //        }
        //
        //        // Set the view to the window as its content view.
        //        [self->window setContentView:self->glView];

        [self->window setDelegate: self];
        [self->window setAcceptsMouseMovedEvents: YES];
        [self->window setIgnoresMouseEvents: NO];

        [self->window center];
        [self->window setAutodisplay: YES];
        [self->window setReleasedWhenClosed: NO];
    }

    return self;
}

- (void) setTitle:(NSString*)title {
    [self->window setTitle: title];
}

- (void) show {
    [self->window makeKeyAndOrderFront: nil];
}

- (NSUInteger) shouldClose {
    return self->shouldClose;
}

- (void) fetchEvents
{
    [NSApplication sharedApplication];
    NSEvent* event = nil;

    while ((event = [NSApp nextEventMatchingMask: NSAnyEventMask
                                       untilDate: [NSDate distantPast]
                                          inMode: NSDefaultRunLoopMode
                                         dequeue: YES])) {
        [NSApp sendEvent:event];
    }
}

- (VEWindow *) getWindow {
    return self->window;
}

// delegate functions

- (BOOL) windowShouldClose:(id)sender {
    self->shouldClose = true;
    return YES;
}

@end

id ve_windowhandler_new(NSSize size, NSUInteger style) {
    return [[VEWindowHandler alloc] initWithSize: size
                                  AndWindowStyle:style];
}

void ve_windowhandler_set_title(id window_handler, const char *title) {
    [window_handler setTitle: [[NSString alloc] initWithUTF8String: title]];
}

void ve_windowhandler_fetch_events(id window_handler) {
    [window_handler fetchEvents];
}

void ve_windowhandler_show(id window_handler) {
    [window_handler show];
}

BOOL ve_windowhandler_should_close(id window_handler) {
    return [window_handler shouldClose];
}
