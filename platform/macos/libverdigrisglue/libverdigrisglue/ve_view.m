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

- (BOOL)canBecomeKeyView {
    return YES;
}

- (BOOL)isOpaque {
    return YES;
}

- (BOOL) acceptsFirstResponder {
    return YES;
}

@end
