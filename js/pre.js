
Module = Module || {};
Module.preRun = Module.preRun || [];
Module.preRun.push(function(){ENV.RUST_BACKTRACE = "short"});

Module.canvas = document.getElementById("main");

var interop = (function(){
	"use strict";
	var ex = {};

	var jsRefs = [];
	var nextRefId = 0;

	function make_ref(val) {
		var id = nextRefId;
		while (jsRefs[id] != null) {
			nextRefId++;
			id = nextRefId;
		}
		nextRefId++;
		jsRefs[id] = val;
		return id;
	}
	function get_ref(ref) {
		return jsRefs[ref];
	}
	ex.drop_ref = function(ref) {
		if (ref < nextRefId) {
			nextRefId = ref;
		}
		delete jsRefs[ref];
	};
	ex.clone_ref = function(ref) {
		var val = get_ref(ref);
		return make_ref(val);
	};

	ex.get_element_by_id = function(id) {
		var elm = document.getElementById(id);
		if (elm == null) return -1;
		return make_ref(elm);
	};
	ex.create_element = function(tag) {
		var elem = document.createElement(tag);
		return make_ref(elem);
	};
	ex.get_body_element = function() {
		return make_ref(document.body);
	};
	ex.append_child = function(aref, bref) {
		get_ref(aref).appendChild(get_ref(bref));
	};
	ex.js_set_prop = function(eref, key, val) {
		var e = get_ref(eref);
		e[key] = val;
	};
	ex.js_set_prop_ref = function(eref, key, vref) {
		var e = get_ref(eref);
		var v = get_ref(vref);
		e[key] = v;
	};
	ex.js_get_prop = function(eref, key) {
		var e = get_ref(eref);
		return e[key];
	};


	ex.get_context_2d = function(cref) {
		var canvas = get_ref(cref);
		return make_ref(canvas.getContext("2d"));
	};
	ex.ctx_fill_rect = function(cref, x, y, w, h) {
		var ctx = get_ref(cref);
		ctx.fillRect(x, y, w, h);
	};
	ex.ctx_draw_image = function(cref, iref, dx, dy) {
		var ctx = get_ref(cref);
		ctx.drawImage(get_ref(iref), dx, dy);
	};
	ex.ctx_draw_image_scaled = function(cref, iref, dx, dy, dw, dh) {
		var ctx = get_ref(cref);
		ctx.drawImage(get_ref(iref), dx, dy, dw, dh);
	};
	ex.ctx_draw_image_part = function(cref, iref, sx, sy, sw, sh, dx, dy, dw, dh) {
		var ctx = get_ref(cref);
		ctx.drawImage(get_ref(iref), sx, sy, sw, sh, dx, dy, dw, dh);
	};
	ex.ctx_fill_text = function(cref, text, x, y) {
		var ctx = get_ref(cref);
		ctx.fillText(text, x, y);
	}
	ex.ctx_save = function(cref) {
		var ctx = get_ref(cref);
		ctx.save();
	};
	ex.ctx_restore = function(cref) {
		var ctx = get_ref(cref);
		ctx.restore();
	};
	ex.ctx_translate = function(cref, x, y) {
		var ctx = get_ref(cref);
		ctx.translate(x, y);
	};
	ex.ctx_scale = function(cref, x, y) {
		var ctx = get_ref(cref);
		ctx.scale(x, y);
	};
	ex.ctx_rotate = function(cref, angle) {
		var ctx = get_ref(cref);
		ctx.rotate(angle);
	};
    ex.ctx_begin_path = function(cref) {
		var ctx = get_ref(cref);
		ctx.beginPath();
    };
    ex.ctx_clip = function(cref) {
		var ctx = get_ref(cref);
		ctx.clip();
    };
    ex.ctx_fill = function(cref) {
		var ctx = get_ref(cref);
		ctx.fill();
    };
    ex.ctx_arc = function(cref, x, y, radius, start, end, anti) {
		var ctx = get_ref(cref);
		ctx.arc(x, y, radius, start, end, anti);
    };
	ex.ctx_create_radial_gradient = function(cref, x0, y0, r0, x1, y1, r1) {
		var ctx = get_ref(cref);
		return make_ref(ctx.createRadialGradient(x0, y0, r0, x1, y1, r1));
	};
	ex.gradient_add_color_stop = function(gref, offset, val) {
		var g = get_ref(gref);
		g.addColorStop(offset, val);
	};

	window.focus();
	document.getElementById("main").addEventListener('mousedown', function(e) {
		window.focus();
		e.preventDefault();
	});

	var eventQueue = [];
	ex.poll_event = function() {
		var evt = eventQueue.shift();
		if (evt == null) {
			return 0;
		}
		return (evt.ty << 16) | (evt.key);
	}
	document.addEventListener("keydown", function(evt) {
		eventQueue.push({
			ty: 1,
			key: evt.which || evt.keyCode || 0,
		});
	});
	document.addEventListener("keyup", function(evt) {
		eventQueue.push({
			ty: 2,
			key: evt.which || evt.keyCode || 0,
		});
	});

	// Audio stuff

	var context = new AudioContext();

	function loadBuffer(url, cb) {
		var xhr = new XMLHttpRequest();
		xhr.open("GET", url, true);
		xhr.responseType = "arraybuffer";

		xhr.onload = function() {
			context.decodeAudioData(xhr.response, function(buffer) {
				cb(buffer);
			})
		};
		xhr.send();
	}
	var shootBuffer;
	loadBuffer("assets/shoot.wav", function(buffer) {
		shootBuffer = buffer;
	});
	var explodeBuffer;
	loadBuffer("assets/explosion.wav", function(buffer) {
		explodeBuffer = buffer;
	})

	ex.play_shoot = function() {
		if (!shootBuffer) return;
		var source = context.createBufferSource();
		source.buffer = shootBuffer;
		source.playbackRate.value = 0.9 + Math.random() * 1.5;
		var gain = context.createGain();
		gain.gain.value = 0.4;
		source.connect(gain);
		gain.connect(context.destination);
		source.start(0);
	};

	ex.play_explode = function() {
		if (!explodeBuffer) return;
		var source = context.createBufferSource();
		source.buffer = explodeBuffer;
		source.playbackRate.value = 0.9 + Math.random() * 1.5;
		var gain = context.createGain();
		gain.gain.value = 0.4;
		source.connect(gain);
		gain.connect(context.destination);
		source.start(0);
	};

	return ex;
})();