/*
 * Copyright © 2011 Benjamin Franzke
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that copyright
 * notice and this permission notice appear in supporting documentation, and
 * that the name of the copyright holders not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  The copyright holders make no representations
 * about the suitability of this software for any purpose.  It is provided "as
 * is" without express or implied warranty.
 *
 * THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <math.h>
#include <assert.h>
#include <signal.h>

#include <linux/input.h>

#include <wayland-client.h>
#include <wayland-egl.h>
#include <wayland-cursor.h>

#include <GLES2/gl2.h>
#include <EGL/egl.h>

// XDG-parts

extern const struct wl_interface wl_output_interface;
extern const struct wl_interface wl_seat_interface;
extern const struct wl_interface wl_surface_interface;
extern const struct wl_interface zxdg_popup_v6_interface;
extern const struct wl_interface zxdg_positioner_v6_interface;
extern const struct wl_interface zxdg_surface_v6_interface;
extern const struct wl_interface zxdg_toplevel_v6_interface;

static const struct wl_interface *types[] = {
	NULL,
	NULL,
	NULL,
	NULL,
	&zxdg_positioner_v6_interface,
	&zxdg_surface_v6_interface,
	&wl_surface_interface,
	&zxdg_toplevel_v6_interface,
	&zxdg_popup_v6_interface,
	&zxdg_surface_v6_interface,
	&zxdg_positioner_v6_interface,
	&zxdg_toplevel_v6_interface,
	&wl_seat_interface,
	NULL,
	NULL,
	NULL,
	&wl_seat_interface,
	NULL,
	&wl_seat_interface,
	NULL,
	NULL,
	&wl_output_interface,
	&wl_seat_interface,
	NULL,
};

static const struct wl_message zxdg_shell_v6_requests[] = {
	{ "destroy", "", types + 0 },
	{ "create_positioner", "n", types + 4 },
	{ "get_xdg_surface", "no", types + 5 },
	{ "pong", "u", types + 0 },
};

static const struct wl_message zxdg_shell_v6_events[] = {
	{ "ping", "u", types + 0 },
};

const struct wl_interface zxdg_shell_v6_interface = {
	"zxdg_shell_v6", 1,
	4, zxdg_shell_v6_requests,
	1, zxdg_shell_v6_events,
};

static const struct wl_message zxdg_positioner_v6_requests[] = {
	{ "destroy", "", types + 0 },
	{ "set_size", "ii", types + 0 },
	{ "set_anchor_rect", "iiii", types + 0 },
	{ "set_anchor", "u", types + 0 },
	{ "set_gravity", "u", types + 0 },
	{ "set_constraint_adjustment", "u", types + 0 },
	{ "set_offset", "ii", types + 0 },
};

const struct wl_interface zxdg_positioner_v6_interface = {
	"zxdg_positioner_v6", 1,
	7, zxdg_positioner_v6_requests,
	0, NULL,
};

static const struct wl_message zxdg_surface_v6_requests[] = {
	{ "destroy", "", types + 0 },
	{ "get_toplevel", "n", types + 7 },
	{ "get_popup", "noo", types + 8 },
	{ "set_window_geometry", "iiii", types + 0 },
	{ "ack_configure", "u", types + 0 },
};

static const struct wl_message zxdg_surface_v6_events[] = {
	{ "configure", "u", types + 0 },
};

const struct wl_interface zxdg_surface_v6_interface = {
	"zxdg_surface_v6", 1,
	5, zxdg_surface_v6_requests,
	1, zxdg_surface_v6_events,
};

static const struct wl_message zxdg_toplevel_v6_requests[] = {
	{ "destroy", "", types + 0 },
	{ "set_parent", "?o", types + 11 },
	{ "set_title", "s", types + 0 },
	{ "set_app_id", "s", types + 0 },
	{ "show_window_menu", "ouii", types + 12 },
	{ "move", "ou", types + 16 },
	{ "resize", "ouu", types + 18 },
	{ "set_max_size", "ii", types + 0 },
	{ "set_min_size", "ii", types + 0 },
	{ "set_maximized", "", types + 0 },
	{ "unset_maximized", "", types + 0 },
	{ "set_fullscreen", "?o", types + 21 },
	{ "unset_fullscreen", "", types + 0 },
	{ "set_minimized", "", types + 0 },
};

static const struct wl_message zxdg_toplevel_v6_events[] = {
	{ "configure", "iia", types + 0 },
	{ "close", "", types + 0 },
};

const struct wl_interface zxdg_toplevel_v6_interface = {
	"zxdg_toplevel_v6", 1,
	14, zxdg_toplevel_v6_requests,
	2, zxdg_toplevel_v6_events,
};

static const struct wl_message zxdg_popup_v6_requests[] = {
	{ "destroy", "", types + 0 },
	{ "grab", "ou", types + 22 },
};

static const struct wl_message zxdg_popup_v6_events[] = {
	{ "configure", "iiii", types + 0 },
	{ "popup_done", "", types + 0 },
};

const struct wl_interface zxdg_popup_v6_interface = {
	"zxdg_popup_v6", 1,
	2, zxdg_popup_v6_requests,
	2, zxdg_popup_v6_events,
};

struct zxdg_surface_v6_listener {
	void (*configure)(void *data,
			  void *zxdg_surface_v6,
			  uint32_t serial);
};

struct zxdg_toplevel_v6_listener {
	void (*configure)(void *data,
			  void *zxdg_toplevel_v6,
			  int32_t width,
			  int32_t height,
			  struct wl_array *states);
	void (*close)(void *data,
		      void *zxdg_toplevel_v6);
};

// Wayland Client

typedef struct Context {
    int32_t running;
    int32_t is_restored;

    int32_t window_width;
    int32_t window_height;

    int32_t restore_width;
    int32_t restore_height;

    uint32_t last_millis;

	GLuint gl_rotation_uniform;
	GLuint gl_pos;
	GLuint gl_col;

	struct wl_egl_window *native;
	struct wl_surface *surface;
	struct wl_shell_surface *shell_surface;
	EGLSurface egl_surface;
	struct wl_callback *callback;
    int configured;
    bool fullscreen;

	struct wl_display *wldisplay;
	struct wl_registry *registry;
	struct wl_compositor *compositor;
	struct wl_shell *shell;
	struct wl_seat *seat;
	struct wl_pointer *pointer;
	struct wl_keyboard *keyboard;
	struct wl_shm *shm;
	struct wl_cursor_theme *cursor_theme;
	struct wl_cursor *default_cursor;
	struct wl_surface *cursor_surface;
    void *toplevel;

	EGLDisplay egl_dpy;
	EGLContext egl_ctx;
	EGLConfig egl_conf;
} Context;

typedef struct OpenGL {
    
} OpenGL;

static const char *vert_shader_text =
	"uniform mat4 rotation;\n"
	"attribute vec4 pos;\n"
	"attribute vec4 color;\n"
	"varying vec4 v_color;\n"
	"void main() {\n"
	"  gl_Position = rotation * pos;\n"
	"  v_color = color;\n"
	"}\n";

static const char *frag_shader_text =
	"precision mediump float;\n"
	"varying vec4 v_color;\n"
	"void main() {\n"
	"  gl_FragColor = v_color;\n"
	"}\n";

static void fini_egl(struct Context *context) {
	/* Required, otherwise segfault in egl_dri2.c: dri2_make_current()
	 * on eglReleaseThread(). */
	eglMakeCurrent(context->egl_dpy, EGL_NO_SURFACE, EGL_NO_SURFACE,
		       EGL_NO_CONTEXT);

	eglTerminate(context->egl_dpy);
	eglReleaseThread();
}

static GLuint create_shader(const char *source, GLenum shader_type) {
	GLuint shader;
	GLint status;

	shader = glCreateShader(shader_type);
	assert(shader != 0);

	glShaderSource(shader, 1, (const char **) &source, NULL);
	glCompileShader(shader);

	glGetShaderiv(shader, GL_COMPILE_STATUS, &status);
	if (!status) {
		char log[1000];
		GLsizei len;
		glGetShaderInfoLog(shader, 1000, &len, log);
		fprintf(stderr, "Error: compiling %s: %*s\n",
			shader_type == GL_VERTEX_SHADER ? "vertex" : "fragment",
			len, log);
		exit(1);
	}

	return shader;
}

static void
init_gl(struct Context *context)
{
	GLuint frag, vert;
	GLuint program;
	GLint status;

	frag = create_shader(frag_shader_text, GL_FRAGMENT_SHADER);
	vert = create_shader(vert_shader_text, GL_VERTEX_SHADER);

	program = glCreateProgram();
	glAttachShader(program, frag);
	glAttachShader(program, vert);
	glLinkProgram(program);

	glGetProgramiv(program, GL_LINK_STATUS, &status);
	if (!status) {
		char log[1000];
		GLsizei len;
		glGetProgramInfoLog(program, 1000, &len, log);
		fprintf(stderr, "Error: linking:\n%*s\n", len, log);
		exit(1);
	}

	glUseProgram(program);
	
	context->gl_pos = 0;
	context->gl_col = 1;

	glBindAttribLocation(program, context->gl_pos, "pos");
	glBindAttribLocation(program, context->gl_col, "color");
	glLinkProgram(program);

	context->gl_rotation_uniform =
		glGetUniformLocation(program, "rotation");
}

void redraw(void *data, struct wl_callback *callback, uint32_t time);

static void
handle_xdg_shell_ping(void *data, void *shell, uint32_t serial)
{
    // PONG
	wl_proxy_marshal((struct wl_proxy *) shell,
        3 /*ZXDG_SHELL_V6_PONG*/, serial);
}

struct zxdg_shell_v6_listener {
	void (*ping)(void *data, void *shell, uint32_t serial);
};

const struct zxdg_shell_v6_listener XDG_SHELL_LISTENER = {
   handle_xdg_shell_ping,
};

void configure_callback(void *data, struct wl_callback *callback, uint32_t time)
{
    struct Context* context = data;

	wl_callback_destroy(callback);

    printf("GL2 %d %d\n", context->window_width, context->window_height);
    glViewport(0, 0, context->window_width, context->window_height);

	if (context->callback == NULL)
		redraw(data, NULL, time);
}

struct wl_callback_listener CONFIGURE_CALLBACK_LISTENER = {
	configure_callback,
};

static void create_surface(struct Context *context) {
	context->native =
		wl_egl_window_create(
            context->surface,
            context->window_width,
            context->window_height
        );

	context->egl_surface =
		eglCreateWindowSurface(
            context->egl_dpy,
            context->egl_conf,
            context->native, NULL
        );
	EGLBoolean ret;
	ret = eglMakeCurrent(context->egl_dpy, context->egl_surface,
			     context->egl_surface, context->egl_ctx);
	assert(ret != 0);
    printf("OOF\n");
}

static void destroy_surface(struct Context *context) {
	wl_surface_destroy(context->surface);
	wl_egl_window_destroy(context->native);

    // Free
	wl_proxy_marshal((struct wl_proxy *) context->shell_surface, 0);
    wl_proxy_destroy((struct wl_proxy *) context->shell_surface);

	if (context->callback)
		wl_callback_destroy(context->callback);
}

static const struct wl_callback_listener frame_listener;

void redraw(void *data, struct wl_callback *callback, uint32_t millis) {
    struct Context* context = data;

	static const GLfloat verts[3][2] = {
		{ -0.5, -0.5 },
		{  0.5, -0.5 },
		{  0,    0.5 }
	};
	static const GLfloat colors[3][3] = {
		{ 1, 0, 0 },
		{ 0, 1, 0 },
		{ 0, 0, 0 }
	};
	GLfloat angle;
	GLfloat rotation[4][4] = {
		{ 1, 0, 0, 0 },
		{ 0, 1, 0, 0 },
		{ 0, 0, 1, 0 },
		{ 0, 0, 0, 1 }
	};
	static const int32_t speed_div = 5;
	static uint32_t start_time = 0;

	assert(context->callback == callback);
	context->callback = NULL;

    uint32_t diff_millis;
    if (callback != NULL) {
        if (start_time == 0) {
            start_time = millis;
            diff_millis = 0;
        } else {
            // TODO: overflowing subtract.
            diff_millis = millis - context->last_millis;
        }

		wl_callback_destroy(callback);
    } else {
        diff_millis = 0;
    }
    // works as long as diff_millis < 4295, should kill process if 200
    // milliseconds passed.  Ideal: 16(64fps)-32(32fps) milliseconds.
    uint32_t diff_nanos = diff_millis * 1000000;
    printf("DIFF_NANOS %d\n", diff_nanos);
    context->last_millis = millis;

	angle = ((millis-start_time) / speed_div) % 360 * M_PI / 180.0;
	rotation[0][0] =  cos(angle);
	rotation[0][2] =  sin(angle);
	rotation[2][0] = -sin(angle);
	rotation[2][2] =  cos(angle);

	glUniformMatrix4fv(context->gl_rotation_uniform, 1, GL_FALSE,
			   (GLfloat *) rotation);

	glClearColor(0.0, 0.0, 1.0, 0.5);
	glClear(GL_COLOR_BUFFER_BIT);

	glVertexAttribPointer(context->gl_pos, 2, GL_FLOAT, GL_FALSE, 0, verts);
	glVertexAttribPointer(context->gl_col, 3, GL_FLOAT, GL_FALSE, 0, colors);
	glEnableVertexAttribArray(context->gl_pos);
	glEnableVertexAttribArray(context->gl_col);

	glDrawArrays(GL_TRIANGLES, 0, 3);

	glDisableVertexAttribArray(context->gl_pos);
	glDisableVertexAttribArray(context->gl_col);

	context->callback = wl_surface_frame(context->surface);
	wl_callback_add_listener(context->callback, &frame_listener, context);

	eglSwapBuffers(context->egl_dpy, context->egl_surface);
}

static const struct wl_callback_listener frame_listener = {
	redraw
};

void dive_wayland(Context* context) {
	create_surface(context);

    printf("WHERE!\n");

	init_gl(context);

    printf("WHY!\n");

	while (context->running) {
		if (wl_display_dispatch(context->wldisplay) == -1) {
            break;
        }
    }

	fprintf(stderr, "simple-egl exiting\n");

	destroy_surface(context);
	fini_egl(context);
}
