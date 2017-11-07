

mergeInto(LibraryManager.library, {
    get_element_by_id: function(id, id_len) {
        return interop.get_element_by_id(Pointer_stringify(id, id_len))
    },
    create_element: function(tag, tag_len) {
        return interop.create_element(Pointer_stringify(tag, tag_len))
    },
    drop_ref: function(ref) {
        interop.drop_ref(ref);
    },
    clone_ref: function(ref) {
        return interop.clone_ref(ref);
    },
    poll_event: function() {
        return interop.poll_event();
    },
    js_set_prop_str: function(eref, key, key_len, val, val_len) {
        interop.js_set_prop(eref, Pointer_stringify(key, key_len), Pointer_stringify(val, val_len));
    },
    js_set_prop_int: function(eref, key, key_len, val) {
        interop.js_set_prop(eref, Pointer_stringify(key, key_len), val);
    },
    js_get_prop_int: function(eref, key, key_len) {
        return interop.js_get_prop(eref, Pointer_stringify(key, key_len));
    },
    js_set_prop_float: function(eref, key, key_len, val) {
        interop.js_set_prop(eref, Pointer_stringify(key, key_len), val);
    },
    js_set_prop_ref: function(eref, key, key_len, oref) {
        interop.js_set_prop_ref(eref, Pointer_stringify(key, key_len), oref);
    },
    get_body_element: function() {
        return interop.get_body_element();
    },
    append_child: function(aref, bref) {
        interop.append_child(aref, bref);
    },
    get_context_2d: function(cref) {
        return interop.get_context_2d(cref);
    },
    ctx_fill_rect: function(cref, x, y, w, h) {
        interop.ctx_fill_rect(cref, x, y, w, h);
    },
    ctx_draw_image: function(cref, iref, dx, dy) {
        interop.ctx_draw_image(cref, iref, dx, dy);
    },
    ctx_draw_image_scaled: function(cref, iref, dx, dy, dw, dh) {
        interop.ctx_draw_image_scaled(cref, iref, dx, dy, dw, dh);
    },
    ctx_draw_image_part: function(cref, iref, sx, sy, sw, sh, dx, dy, dw, dh) {
        interop.ctx_draw_image_part(cref, iref, sx, sy, sw, sh, dx, dy, dw, dh);
    },
    ctx_fill_text: function(cref, text, text_len, x, y) {
        interop.ctx_fill_text(cref, Pointer_stringify(text, text_len), x, y)
    },
    ctx_save: function(cref) {
        interop.ctx_save(cref);
    },
    ctx_restore: function(cref) {
        interop.ctx_restore(cref);
    },
    ctx_translate: function(cref, x, y) {
        interop.ctx_translate(cref, x, y);
    },
    ctx_scale: function(cref, x, y) {
        interop.ctx_scale(cref, x, y);
    },
    ctx_rotate: function(cref, angle) {
        interop.ctx_rotate(cref, angle);
    },
    ctx_begin_path: function(cref) {
        interop.ctx_begin_path(cref);
    },
    ctx_clip: function(cref) {
        interop.ctx_clip(cref);
    },
    ctx_fill: function(cref) {
        interop.ctx_fill(cref);
    },
    ctx_arc: function(cref, x, y, radius, start, end, anti) {
        interop.ctx_arc(cref, x, y, radius, start, end, anti);
    },
    ctx_create_radial_gradient: function(cref, x0, y0, r0, x1, y1, r1) {
        return interop.ctx_create_radial_gradient(cref, x0, y0, r0, x1, y1, r1);
    },
    gradient_add_color_stop: function(gref, offset, val, val_len) {
        return interop.gradient_add_color_stop(gref, offset, Pointer_stringify(val, val_len));
    },

    play_shoot: function() {
        interop.play_shoot();
    },
    play_explode: function() {
        interop.play_explode();
    },

    // Hacks to get backtraces into a somewhat useful state
    _Unwind_Backtrace__deps: ['emscripten_get_callstack_js'],
    _Unwind_Backtrace: function(func, arg) {
        Error.stackTraceLimit = Infinity;
        var trace = _emscripten_get_callstack_js();
        var parts = trace.split('\n');
        for (var i = 0; i < parts.length; i++) {
            var line = parts[i];
            var atPos = line.indexOf("at") + 3;
            var desc = line.substring(atPos, line.indexOf("(") - 1);
            if (desc.indexOf("Array.") == 0) {
                desc = desc.substring(6);
            }
            if (desc.indexOf("__ZN") != 0) {
                continue;
            }
            desc = desc.substring(1);
            var ptr = Module._malloc(desc.length + 1);
            Module.stringToUTF8(desc, ptr, desc.length + 1);

            var ret = Runtime.dynCall('iii', func, [ptr, arg]);
            // Module._free(ptr);
            if (ret !== 0) return;
        }
        console.trace();
        return 5;
    },
    _Unwind_FindEnclosingFunction: function(ctx) {
        return ctx;
    },
    _Unwind_GetIPInfo: function(ctx) {
        return ctx;
    },
    dladdr: function(addr, info) {
        var line = UTF8ToString(addr + 1);
        var ptr = Module._malloc(line.length + 1);
        Module.stringToUTF8(line, ptr, line.length + 1);
        Module.setValue(info + 8, ptr, "*");
        return 1;
    },

    emscripten_get_mouse_status_fixed: function(mouseState) {
        if (!JSEvents.mouseEvent) return -7;
        // HTML5 does not really have a polling API for mouse events, so implement one manually by
        // returning the data from the most recently received event. This requires that user has registered
        // at least some no-op function as an event handler to any of the mouse function.
        HEAP8.set(HEAP8.subarray(JSEvents.mouseEvent, JSEvents.mouseEvent + 72), mouseState);
        return 0;
    },
});
