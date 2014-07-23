//
//  NSObject+VECursor.m
//  libverdigrisglue
//
//  Created by Jeremy on 21/07/2014.
//  Copyright (c) 2014 libverdigris. All rights reserved.
//

#import "VECursor.h"

void ve_cursor_show() {
    [NSCursor unhide];
}

void ve_cursor_hide() {
    [NSCursor hide];
}

void ve_cursor_set(NSUInteger cursor) {
    NSLog(@"%d", (int)cursor);
    [NSCursor pop];
//    if (cursor == 2) {
//
//    }
    switch ((int)cursor) {
        case 0: [[NSCursor arrowCursor] push]; NSLog(@"OKAY1");break;
        case 1: [[NSCursor IBeamCursor] push]; break;
        case 2: [[[NSCursor crosshairCursor] init] push]; NSLog(@"OKAY2"); break;
        case 3: [[NSCursor closedHandCursor] push]; NSLog(@"OKAY");break;
        case 4: [[NSCursor openHandCursor] push]; break;
        case 5: [[NSCursor pointingHandCursor] push]; break;
        case 6: [[NSCursor resizeLeftCursor] push]; break;
        case 7: [[NSCursor resizeRightCursor] push]; break;
        case 8: [[NSCursor resizeLeftRightCursor] push]; break;
        case 9: [[NSCursor resizeUpCursor] push]; break;
        case 10: [[NSCursor resizeUpDownCursor] push]; break;
        case 11: [[NSCursor disappearingItemCursor] push]; break;
        case 12: [[NSCursor IBeamCursorForVerticalLayout] push]; break;
        default: NSLog(@"DEFAULT");
            break;
    }
    NSLog(@"OUT");
}