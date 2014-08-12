//
//  VEView.m
//  libverdigrisglue
//
//  Created by Jeremy on 12/08/2014.
//  Copyright (c) 2014 libverdigris. All rights reserved.
//

#import "ve_view.h"

@implementation VEView


- (id)initWithFrame:(NSRect)frame
{
    self = [super initWithFrame:frame];
    return self;
}

//- (void) _surfaceNeedsUpdate:(NSNotification*)notification
//{
//    [self update];
//}

- (BOOL)canBecomeKeyView {
    return YES;
}

- (BOOL)isOpaque {
    return YES;
}

- (BOOL) acceptsFirstResponder {
    return YES;
}

//- (void)setOpenGLContext:(NSOpenGLContext*)context {
//    self->_openGLContext = context;
//}
//
//- (NSOpenGLContext*)openGLContext {
//    return self->_openGLContext;
//}
//
//- (void)clearGLContext {
//
//}
//
//- (void)prepareOpenGL {
//
//}
//
//
//- (void)update {
//    [self->_openGLContext update];
//}
//
//- (void)setPixelFormat:(NSOpenGLPixelFormat*)pixelFormat {
//    self->_pixelFormat = pixelFormat;
//}
//
//- (NSOpenGLPixelFormat*)pixelFormat {
//    return self->_pixelFormat;
//}
//
@end
