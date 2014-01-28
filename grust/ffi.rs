use types::*;
use plumbing::{GMainContext,GMainLoop,GTypeInstance};
use object::GType;

#[link(name="grustna")]
extern {
    pub fn grustna_call(func: extern "C" fn(*(), *GMainContext),
                    data: *(),
                    context: *GMainContext) -> gboolean;
    pub fn grustna_main_loop_new_thread_local() -> *GMainLoop;
    pub fn grustna_main_loop_run_thread_local(l: *GMainLoop);
}

#[link(name="glib-2.0")]
extern {
    pub fn g_free(mem: *());
    pub fn g_strdup(str: *gchar) -> *gchar;
    pub fn g_main_context_ref(context: *GMainContext) -> *GMainContext;
    pub fn g_main_context_unref(context: *GMainContext);
    pub fn g_main_loop_ref(l: *GMainLoop) -> *GMainLoop;
    pub fn g_main_loop_unref(l: *GMainLoop);
    pub fn g_main_loop_run(l: *GMainLoop);
    pub fn g_main_loop_quit(l: *GMainLoop);
}

#[link(name="gobject-2.0")]
extern {
    pub fn g_type_init();
    pub fn g_object_ref(obj: *()) -> *();
    pub fn g_object_unref(obj: *()) -> *();
    pub fn g_type_check_instance_is_a(instance   : *GTypeInstance,
                                  iface_type : GType) -> gboolean;
    pub fn g_type_name(t: GType) -> *gchar;
}
