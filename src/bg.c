
#include<stdio.h>
#include <Imlib2.h>
#include <X11/Xatom.h>
#include <X11/Xlib.h>

int load_image(const char *file_name, int alpha, Imlib_Image rootimg, int width,
               int height) {
  int imgW, imgH, o;

  Imlib_Image buffer = imlib_load_image(file_name);

  if (!buffer)
    return 0;

  imlib_context_set_image(buffer);
  imgW = imlib_image_get_width();
  imgH = imlib_image_get_height();

  imlib_context_set_image(rootimg);

  printf("output %d: size(%d, %d) pos(%d, %d)\n", 0, width, height, 0, 0);
  imlib_context_set_cliprect(0, 0, width, height);

  imlib_blend_image_onto_image(buffer, 0, 0, 0, imgW, imgH, 0, 0, width,
                               height);

  imlib_context_set_image(buffer);
  imlib_free_image();

  imlib_context_set_image(rootimg);

  return 1;
}

int setRootAtoms(Display *display, int screen, Pixmap pixmap) {
  Atom atom_root, atom_eroot, type;
  unsigned char *data_root, *data_eroot;
  int format;
  unsigned long length, after;

  atom_root = XInternAtom(display, "_XROOTPMAP_ID", True);
  atom_eroot = XInternAtom(display, "ESETROOT_PMAP_ID", True);

  // doing this to clean up after old background
  if (atom_root != None && atom_eroot != None) {
    XGetWindowProperty(display, RootWindow(display, screen), atom_root, 0L, 1L,
                       False, AnyPropertyType, &type, &format, &length, &after,
                       &data_root);

    if (type == XA_PIXMAP) {
      XGetWindowProperty(display, RootWindow(display, screen), atom_eroot, 0L,
                         1L, False, AnyPropertyType, &type, &format, &length,
                         &after, &data_eroot);

      if (data_root && data_eroot && type == XA_PIXMAP &&
          *((Pixmap *)data_root) == *((Pixmap *)data_eroot));
//        XKillClient(display, *((Pixmap *)data_root));
    }
  }

  atom_root = XInternAtom(display, "_XROOTPMAP_ID", False);
  atom_eroot = XInternAtom(display, "ESETROOT_PMAP_ID", False);

  if (atom_root == None || atom_eroot == None)
    return 0;

  // setting new background atoms
  XChangeProperty(display, RootWindow(display, screen), atom_root, XA_PIXMAP,
                  32, PropModeReplace, (unsigned char *)&pixmap, 1);
  XChangeProperty(display, RootWindow(display, screen), atom_eroot, XA_PIXMAP,
                  32, PropModeReplace, (unsigned char *)&pixmap, 1);

  return 1;
}

//int main(int argc, char **argv) {

int set_background_x11(char * file_name){
  // Globals:
  Display *display = XOpenDisplay(NULL);
  int screen = XDefaultScreen(display);
  Visual *vis = DefaultVisual(display, screen);
  int cm = DefaultColormap(display, screen);
  int width = DisplayWidth(display, screen);
  int height = DisplayHeight(display, screen);
  int depth = DefaultDepth(display, screen);

  Imlib_Context *context = imlib_context_new();
  imlib_context_push(context);
  imlib_context_set_display(display);

  Pixmap pixmap = XCreatePixmap(display, RootWindow(display, screen), width, height, depth);
  imlib_context_set_visual(vis);
  imlib_context_set_colormap(cm);
  imlib_context_set_drawable(pixmap);
  imlib_context_set_color_range(imlib_create_color_range());

  Imlib_Image image = imlib_create_image(width, height);

    load_image(file_name, 255, image, width, height);
    int imgW = imlib_image_get_width();
    int imgH = imlib_image_get_height();
    imlib_context_set_cliprect(0, 0, width, height);
    imlib_blend_image_onto_image(image, 0, 0, 0, imgW, imgH, 0, 0, width,height);

    imlib_render_image_on_drawable(0, 0);
    setRootAtoms(display, screen, pixmap);

    XFlush(display);
  imlib_free_color_range();
  imlib_free_image();
  imlib_context_pop();
  imlib_context_free(context);
  //
  setRootAtoms(display, screen, pixmap);

  XKillClient(display, AllTemporary);
  XSetCloseDownMode(display, RetainTemporary);

   XSetWindowBackgroundPixmap(display, RootWindow(display, screen), pixmap);
   XClearWindow(display, RootWindow(display, screen));

  XFlush(display);
  XSync(display, False);

  return 0;
}