//
//  VEView.h
//  libverdigrisglue
//
//  Created by Jeremy on 12/08/2014.
//  Copyright (c) 2014 libverdigris. All rights reserved.
//

#import <Cocoa/Cocoa.h>

@interface VEView : NSView {
    @private
        NSOpenGLContext*     _openGLContext;
        NSOpenGLPixelFormat* _pixelFormat;
}

- (id)initWithFrame:(NSRect)frameRect;
- (BOOL) acceptsFirstResponder;
- (BOOL)canBecomeKeyView;
- (BOOL)isOpaque;

@end
