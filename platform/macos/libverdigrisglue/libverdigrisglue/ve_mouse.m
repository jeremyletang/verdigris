//
//  VEMouse.m
//  libverdigrisglue
//
//  Created by Jeremy on 23/07/2014.
//  Copyright (c) 2014 libverdigris. All rights reserved.
//

#import "ve_mouse.h"

NSPoint ve_mouse_get_global_location() {
    return [NSEvent mouseLocation];
}

NSPoint ve_mouse_get_location(id window_handler) {
    return [[window_handler getWindow] mouseLocationOutsideOfEventStream];
}
