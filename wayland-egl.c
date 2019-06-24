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

// Wayland Client

typedef struct Context {
    int32_t running;
    int32_t is_restored;

    int32_t window_width;
    int32_t window_height;

    int32_t restore_width;
    int32_t restore_height;

    uint32_t last_millis;

	struct wl_egl_window *native;
	struct wl_surface *surface;
	struct wl_shell_surface *shell_surface;
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
} Context;

typedef struct OpenGL {
	EGLSurface egl_surface;
	EGLDisplay egl_dpy;
	EGLContext egl_ctx;
	EGLConfig egl_conf;

	GLint gl_rotation_uniform;
	GLuint gl_pos;
	GLuint gl_col;
} OpenGL;

typedef struct WaylandOpenGL {
    Context* wayland;
    OpenGL* opengl;
} WaylandOpenGL;

static const struct wl_callback_listener frame_listener;

void redraw(void *data, struct wl_callback *callback, uint32_t millis) {
    struct WaylandOpenGL* context = data;

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

	assert(context->wayland->callback == callback);
	context->wayland->callback = NULL;

    uint32_t diff_millis;
    if (callback != NULL) {
        if (start_time == 0) {
            start_time = millis;
            diff_millis = 0;
        } else {
            // TODO: overflowing subtract.
            diff_millis = millis - context->wayland->last_millis;
        }

		wl_callback_destroy(callback);
    } else {
        diff_millis = 0;
    }
    // works as long as diff_millis < 4295, should kill process if 200
    // milliseconds passed.  Ideal: 16(64fps)-32(32fps) milliseconds.
    uint32_t diff_nanos = diff_millis * 1000000;
//    printf("DIFF_NANOS %d\n", diff_nanos);
    context->wayland->last_millis = millis;

	angle = ((millis-start_time) / speed_div) % 360 * M_PI / 180.0;
	rotation[0][0] =  cos(angle);
	rotation[0][2] =  sin(angle);
	rotation[2][0] = -sin(angle);
	rotation[2][2] =  cos(angle);

	glUniformMatrix4fv(context->opengl->gl_rotation_uniform, 1, GL_FALSE,
			   (GLfloat *) rotation);

	glClearColor(0.0, 0.0, 1.0, 0.5);
	glClear(GL_COLOR_BUFFER_BIT);

	glVertexAttribPointer(context->opengl->gl_pos, 2, GL_FLOAT, GL_FALSE, 0, verts);
	glVertexAttribPointer(context->opengl->gl_col, 3, GL_FLOAT, GL_FALSE, 0, colors);
	glEnableVertexAttribArray(context->opengl->gl_pos);
	glEnableVertexAttribArray(context->opengl->gl_col);

	glDrawArrays(GL_TRIANGLES, 0, 3);

	glDisableVertexAttribArray(context->opengl->gl_pos);
	glDisableVertexAttribArray(context->opengl->gl_col);

	context->wayland->callback = wl_surface_frame(context->wayland->surface);
	wl_callback_add_listener(context->wayland->callback, &frame_listener, context);

	eglSwapBuffers(context->opengl->egl_dpy, context->opengl->egl_surface);
}

static const struct wl_callback_listener frame_listener = {
	redraw
};

void dive_wayland(WaylandOpenGL* wayland_opengl) {
    printf("WHY!\n");

	while (wayland_opengl->wayland->running) {
		if (wl_display_dispatch(wayland_opengl->wayland->wldisplay) == -1) {
            break;
        }
    }
}