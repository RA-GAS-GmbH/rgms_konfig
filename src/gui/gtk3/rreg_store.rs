use crate::{registers::Register, platine::BoxedPlatine};
use gtk::prelude::*;

/// GtkTreestore for a Rwreg
pub struct RregStore {
    store: gtk::TreeStore,
}

impl RregStore {
    /// Erstellt eine neuen RregStore
    pub fn new() -> Self {
        let store = gtk::TreeStore::new(&[
            // Rreg Nr.
            glib::Type::U32,
            // Wertebereich
            glib::Type::String,
            // Zugeordnete Größe und Einheit
            glib::Type::String,
            // Messwerteigenschaft
            glib::Type::String,
        ]);

        RregStore { store }
    }

    /// Füllt den TreeStore mit Daten
    fn fill_treestore(&self, platine: BoxedPlatine) {
        for reg in &*platine.lock().unwrap().as_ref().unwrap().rregs() {
            self.store.insert_with_values(
                None,
                None,
                &[0, 1, 2, 3],
                &[
                    &reg.reg_nr(),
                    &reg.range(),
                    &reg.value(),
                    &reg.description(),
                ],
            );
        }
    }

    /// Füllt den TreeStore mit Daten und buildet die GUI Komponenten
    pub fn fill_and_build_ui(&self, platine: BoxedPlatine) -> gtk::ScrolledWindow {
        self.fill_treestore(platine);
        let sortable_store = gtk::TreeModelSort::new(&self.store);
        let treeview = gtk::TreeView::with_model(&sortable_store);

        // Renderer Column 0
        let column_reg = gtk::TreeViewColumn::new();
        column_reg.set_title("Rwreg Nr.");
        column_reg.set_clickable(false);
        column_reg.set_sort_indicator(true);
        column_reg.set_sort_column_id(0);
        let renderer = gtk::CellRendererText::new();
        column_reg.pack_end(&renderer, true);
        column_reg.add_attribute(&renderer, "text", 0);
        treeview.append_column(&column_reg);

        // Renderer Column 1
        let column_range = gtk::TreeViewColumn::new();
        column_range.set_title("Wertebereich");
        let renderer = gtk::CellRendererText::new();
        column_range.pack_end(&renderer, true);
        column_range.add_attribute(&renderer, "text", 1);
        treeview.append_column(&column_range);

        // Renderer Column 2
        let column_value = gtk::TreeViewColumn::new();
        column_value.set_title("Zugeordnete Größe und Einheit");
        let renderer = gtk::CellRendererText::new();
        renderer.set_property_editable(false);
        column_value.pack_end(&renderer, true);
        column_value.add_attribute(&renderer, "text", 2);
        treeview.append_column(&column_value);

        // Renderer Column 3
        let column_property = gtk::TreeViewColumn::new();
        column_property.set_title("Messwerteigenschaft");
        let renderer = gtk::CellRendererText::new();
        column_property.pack_end(&renderer, true);
        column_property.add_attribute(&renderer, "text", 3);
        treeview.append_column(&column_property);

        // Scrolled window
        let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_window.add(&treeview);

        scrolled_window
    }

    /// Update TreeStore
    pub fn update_treestore(&self, _new_values: &[u16]) {}
}
