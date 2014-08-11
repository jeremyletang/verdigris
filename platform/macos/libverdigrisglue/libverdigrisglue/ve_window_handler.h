//
//  ve_window_handler.h
//  libverdigrisglue
//
//  Created by Jeremy on 07/07/2014.
//  Copyright (c) 2014 libverdigris. All rights reserved.
//

#import <Cocoa/Cocoa.h>
#import "ve_window.h"

id ve_windowhandler_new(NSSize size, NSUInteger style);
void ve_windowhandler_set_title(id window_handler, const char *title);
void ve_windowhandler_fetch_events(id window_handler);
void ve_windowhandler_show(id window_handler);
BOOL ve_windowhandler_should_close(id window_handler);

@interface VEWindowHandler : NSObject<NSWindowDelegate> {
    VEWindow *window;
    // FIXME should be removed and add in the events queue
    BOOL shouldClose;
}

- (id) initWithSize:(NSSize)size AndWindowStyle:(NSUInteger)style;
- (void) setTitle:(NSString*)title;
- (NSUInteger) shouldClose;
- (void) show;
- (VEWindow *) getWindow;

- (void) fetchEvents;

@end
