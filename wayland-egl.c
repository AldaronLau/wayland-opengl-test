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
#include <math.h>
#include <assert.h>
#include <signal.h>

#include <linux/input.h>

#include <wayland-client.h>
#include <wayland-egl.h>
#include <wayland-cursor.h>

#include <GLES2/gl2.h>
#include <EGL/egl.h>

typedef struct Context {
    int running;

    int window_width;
    int window_height;

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

	EGLDisplay egl_dpy;
	EGLContext egl_ctx;
	EGLConfig egl_conf;
} Context;

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

static void init_egl(struct Context *context) {
	static const EGLint context_attribs[] = {
		EGL_CONTEXT_CLIENT_VERSION, 2,
		EGL_NONE
	};

	EGLint config_attribs[] = {
		EGL_SURFACE_TYPE, EGL_WINDOW_BIT,
		EGL_RED_SIZE, 8,
		EGL_GREEN_SIZE, 8,
		EGL_BLUE_SIZE, 8,
//		EGL_ALPHA_SIZE, 1,
		EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT,
		EGL_NONE
	};

	EGLint major, minor, n;
	EGLBoolean ret;

	context->egl_dpy = eglGetDisplay(context->wldisplay);
	assert(context->egl_dpy);

	ret = eglInitialize(context->egl_dpy, &major, &minor);
	assert(ret == EGL_TRUE);
	ret = eglBindAPI(EGL_OPENGL_ES_API);
	assert(ret == EGL_TRUE);

	ret = eglChooseConfig(context->egl_dpy, config_attribs,
			      &context->egl_conf, 1, &n);
	assert(ret && n == 1);

	context->egl_ctx = eglCreateContext(context->egl_dpy,
					    context->egl_conf,
					    EGL_NO_CONTEXT, context_attribs);
	assert(context->egl_ctx);
}

static void
fini_egl(struct Context *context)
{
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

static void
handle_ping(void *data, struct wl_shell_surface *shell_surface,
	    uint32_t serial)
{
	wl_shell_surface_pong(shell_surface, serial);
}

static void redraw(void *data, struct wl_callback *callback, uint32_t time);

static void
configure_callback(void *data, struct wl_callback *callback, uint32_t  time)
{
    struct Context* context = data;

	wl_callback_destroy(callback);

    printf("GL2 %d %d\n", context->window_width, context->window_height);
    glViewport(0, 0, context->window_width, context->window_height);

	if (context->callback == NULL)
		redraw(data, NULL, time);
}

static struct wl_callback_listener configure_callback_listener = {
	configure_callback,
};

static void handle_configure(void *data, struct wl_shell_surface *shell_surface,
    uint32_t edges, int32_t width, int32_t height)
{
    struct Context* c = data;

	if (c->native && c->configured) {
		wl_egl_window_resize(c->native, width, height, 0, 0);
        c->configured = 0;
        c->window_width = width;
        c->window_height = height;
    } else {
        printf("GL %d %d\n", c->window_width, c->window_height);
	    glViewport(0, 0, c->window_width, c->window_height);
    }
}

static void handle_popup_done(void *data, struct wl_shell_surface *shell_surface) {
}

static const struct wl_shell_surface_listener shell_surface_listener = {
	handle_ping,
	handle_configure,
	handle_popup_done
};

static void create_surface(struct Context *context) {
	EGLBoolean ret;
	
	context->surface = wl_compositor_create_surface(context->compositor);
	context->shell_surface = wl_shell_get_shell_surface(context->shell,
							   context->surface);

	wl_shell_surface_add_listener(context->shell_surface,
				      &shell_surface_listener, context);

	context->native =
		wl_egl_window_create(context->surface,
				     context->window_width,
				     context->window_height);
	context->egl_surface =
		eglCreateWindowSurface(context->egl_dpy,
				       context->egl_conf,
				       context->native, NULL);

	wl_shell_surface_set_title(context->shell_surface, "simple-egl");

	ret = eglMakeCurrent(context->egl_dpy, context->egl_surface,
			     context->egl_surface, context->egl_ctx);
	assert(ret == EGL_TRUE);

    // Maximize Window.
	wl_shell_surface_set_maximized(
        context->shell_surface,
        NULL
    );

	struct wl_callback *callback = wl_display_sync(context->wldisplay);
	wl_callback_add_listener(callback, &configure_callback_listener, context);
}

static void destroy_surface(struct Context *context) {
	wl_egl_window_destroy(context->native);

	wl_shell_surface_destroy(context->shell_surface);
	wl_surface_destroy(context->surface);

	if (context->callback)
		wl_callback_destroy(context->callback);
}

static const struct wl_callback_listener frame_listener;

static void redraw(void *data, struct wl_callback *callback, uint32_t time) {
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
//	struct wl_region *region;

	assert(context->callback == callback);
	context->callback = NULL;

	if (callback)
		wl_callback_destroy(callback);

	if (start_time == 0)
		start_time = time;

	angle = ((time-start_time) / speed_div) % 360 * M_PI / 180.0;
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

static void
pointer_handle_enter(void *data, struct wl_pointer *pointer,
		     uint32_t serial, struct wl_surface *surface,
		     wl_fixed_t sx, wl_fixed_t sy)
{
/*	struct display *display = data;
	struct wl_buffer *buffer;
	struct wl_cursor *cursor = display->default_cursor;
	struct wl_cursor_image *image;

    // Hide cursor
	wl_pointer_set_cursor(pointer, serial, NULL, 0, 0);*/
}

static void
pointer_handle_leave(void *data, struct wl_pointer *pointer,
		     uint32_t serial, struct wl_surface *surface)
{
}

static void
pointer_handle_motion(void *data, struct wl_pointer *pointer,
		      uint32_t time, wl_fixed_t sx, wl_fixed_t sy)
{
}

static void
pointer_handle_button(void *data, struct wl_pointer *wl_pointer,
		      uint32_t serial, uint32_t time, uint32_t button,
		      uint32_t state)
{
    struct Context* c = data;

/*	if (button == BTN_LEFT && state == WL_POINTER_BUTTON_STATE_PRESSED)
		wl_shell_surface_move(display->window->shell_surface,
				      display->seat, serial);*/
}

static void
pointer_handle_axis(void *data, struct wl_pointer *wl_pointer,
		    uint32_t time, uint32_t axis, wl_fixed_t value)
{
}

static const struct wl_pointer_listener pointer_listener = {
	pointer_handle_enter,
	pointer_handle_leave,
	pointer_handle_motion,
	pointer_handle_button,
	pointer_handle_axis,
};

static void
keyboard_handle_keymap(void *data, struct wl_keyboard *keyboard,
		       uint32_t format, int fd, uint32_t size)
{
}

static void
keyboard_handle_enter(void *data, struct wl_keyboard *keyboard,
		      uint32_t serial, struct wl_surface *surface,
		      struct wl_array *keys)
{
}

static void
keyboard_handle_leave(void *data, struct wl_keyboard *keyboard,
		      uint32_t serial, struct wl_surface *surface)
{
}

static void
keyboard_handle_key(void *data, struct wl_keyboard *keyboard,
		    uint32_t serial, uint32_t time, uint32_t key,
		    uint32_t state)
{
    struct Context* c = data;

	if (key == KEY_ESC && state)
		c->running = 0;
    else if (key == KEY_F11 && state) {
        c->configured = 1;

        if(c->fullscreen) {
	        wl_shell_surface_set_maximized(
                c->shell_surface,
                NULL
            );
		    handle_configure(c, c->shell_surface, 0,
				 c->window_width, c->window_height);

            c->fullscreen = false;
        } else {
        	wl_shell_surface_set_fullscreen(
                c->shell_surface,
                WL_SHELL_SURFACE_FULLSCREEN_METHOD_DEFAULT,
                0, NULL
            );

            c->fullscreen = true;
        }

        struct wl_callback *callback = wl_display_sync(c->wldisplay);
        wl_callback_add_listener(callback, &configure_callback_listener, c);
    }
}

static void
keyboard_handle_modifiers(void *data, struct wl_keyboard *keyboard,
			  uint32_t serial, uint32_t mods_depressed,
			  uint32_t mods_latched, uint32_t mods_locked,
			  uint32_t group)
{
}

static const struct wl_keyboard_listener keyboard_listener = {
	keyboard_handle_keymap,
	keyboard_handle_enter,
	keyboard_handle_leave,
	keyboard_handle_key,
	keyboard_handle_modifiers,
};

static void
seat_handle_capabilities(void *data, struct wl_seat *seat,
			 enum wl_seat_capability caps)
{
    struct Context* c = data;

    // Allow Pointer Events
	if ((caps & WL_SEAT_CAPABILITY_POINTER) && !c->pointer) {
		c->pointer = wl_seat_get_pointer(seat);
		wl_pointer_add_listener(c->pointer, &pointer_listener, c);
	} else if (!(caps & WL_SEAT_CAPABILITY_POINTER) && c->pointer) {
		wl_pointer_destroy(c->pointer);
		c->pointer = NULL;
	}

    // Allow Keyboard Events
	if ((caps & WL_SEAT_CAPABILITY_KEYBOARD) && !c->keyboard) {
		c->keyboard = wl_seat_get_keyboard(seat);
		wl_keyboard_add_listener(c->keyboard, &keyboard_listener, c);
	} else if (!(caps & WL_SEAT_CAPABILITY_KEYBOARD) && c->keyboard) {
		wl_keyboard_destroy(c->keyboard);
		c->keyboard = NULL;
	}
}

static const struct wl_seat_listener seat_listener = {
	seat_handle_capabilities,
};

static void
registry_handle_global(void *data, struct wl_registry *registry,
		       uint32_t name, const char *interface, uint32_t version)
{
    struct Context* c = data;

	if (strcmp(interface, "wl_compositor") == 0) {
		c->compositor =
			wl_registry_bind(registry, name,
					 &wl_compositor_interface, 1);
	} else if (strcmp(interface, "wl_shell") == 0) {
		c->shell = wl_registry_bind(registry, name,
					    &wl_shell_interface, 1);
	} else if (strcmp(interface, "wl_seat") == 0) {
		c->seat = wl_registry_bind(registry, name,
					   &wl_seat_interface, 1);
		wl_seat_add_listener(c->seat, &seat_listener, c);
	} else if (!strcmp(interface, "wl_shm")) {
        c->shm = wl_registry_bind (registry, name, &wl_shm_interface, 1);
	}
}

static void registry_handle_global_remove(void *data,
    struct wl_registry *registry, uint32_t name)
{
}

static const struct wl_registry_listener registry_listener = {
	registry_handle_global,
	registry_handle_global_remove
};

void dive_wayland(Context* context) {
//	context->registry = wl_display_get_registry(context->wldisplay);

	wl_registry_add_listener(context->registry,
				 &registry_listener, context);

	wl_display_dispatch(context->wldisplay);

	init_egl(context);
	create_surface(context);
	init_gl(context);

	context->cursor_surface =
		wl_compositor_create_surface(context->compositor);

	while (context->running) {
		if (wl_display_dispatch(context->wldisplay) == -1) {
            break;
        }
    }

	fprintf(stderr, "simple-egl exiting\n");

	destroy_surface(context);
	fini_egl(context);

	wl_surface_destroy(context->cursor_surface);
	if (context->cursor_theme)
		wl_cursor_theme_destroy(context->cursor_theme);

	if (context->shell)
		wl_shell_destroy(context->shell);

	if (context->compositor)
		wl_compositor_destroy(context->compositor);

	wl_registry_destroy(context->registry);
}
