//
//  VEWindowHandler.h
//  VerdigrisCocoaGlue
//
//  Created by Jeremy on 07/07/2014.
//  Copyright (c) 2014 libverdigris. All rights reserved.
//

#import <Cocoa/Cocoa.h>
#import "VEWindow.h"

@interface VEWindowHandler : NSObject<NSWindowDelegate> {
    VEWindow *window;
    // FIXME should be removed and add in the events queue
    BOOL shouldClose;
}

- (id) initWithSize:(NSSize)size AndWindowStyle:(NSUInteger)style;
- (void) setTitle:(NSString*)title;
- (NSUInteger) shouldClose;
- (void) show;

- (void) fetchEvents;

@end
