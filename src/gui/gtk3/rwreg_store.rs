use crate::platine::Platine;
use gtk::prelude::*;

/// GtkTreestore for a Rwreg
pub struct RwregStore {
    store: gtk::TreeStore,
}

impl RwregStore {
    /// Erstellt eine neuen RwregStore
    pub fn new() -> Self {
        let store = gtk::TreeStore::new(&[
            // Rweg Nr.
            glib::Type::U32,
            // Wertebereich
            glib::Type::String,
            // Zugeordnete Größe und Einheit
            glib::Type::String,
            // Messwerteigenschaft
            glib::Type::String,
            // protected
            glib::Type::Bool,
        ]);

        RwregStore { store }
    }

    /// Füllt den TreeStore mit Daten
    fn fill_treestore(&self, platine: Box<dyn Platine>) {
        for reg in platine.rwregs() {
            self.store.insert_with_values(
                None,
                None,
                &[0, 1, 2, 3, 4],
                &[
                    &reg.rwreg_nr(),
                    &reg.range(),
                    &reg.value(),
                    &reg.description(),
                    &reg.is_protected(),
                ],
            );
        }
    }

    /// Buildet die GUI Komponenten
    pub fn build_ui(&self, platine: Box<dyn Platine>) -> gtk::ScrolledWindow {
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
        renderer.set_property_editable(true);
        column_value.pack_end(&renderer, true);
        column_value.add_attribute(&renderer, "text", 2);
        treeview.append_column(&column_value);
        // Callbacks
        let store = self.store.clone();
        renderer.connect_edited(move |_widget, path, text| {
            // debug!("Edited:\nwidget: {:?}\npath: {:?}\ntext: {:?}\n", widget, path, text);
            callback_edit_cell(&path, text, &store);
        });

        // Renderer Column 3
        let column_property = gtk::TreeViewColumn::new();
        column_property.set_title("Messwerteigenschaft");
        let renderer = gtk::CellRendererText::new();
        column_property.pack_end(&renderer, true);
        column_property.add_attribute(&renderer, "text", 3);
        treeview.append_column(&column_property);

        // 2020-09-15 20:10:18 smueller: Auskommentiert weil es doof aussieht
        //
        // // Renderer Column 4
        // let column_property = gtk::TreeViewColumn::new();
        // column_property.set_title("gesichert");
        // let renderer = gtk::CellRendererText::new();
        // column_property.pack_end(&renderer, true);
        // column_property.add_attribute(&renderer, "text", 4);
        // treeview.append_column(&column_property);

        // Scrolled window
        let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_window.add(&treeview);

        scrolled_window
    }

    /// Update TreeStore
    pub fn update_treestore(&self, _new_values: &[u16]) {}
}

/// callback called if a editable cell is updated with new value
fn callback_edit_cell(path: &gtk::TreePath, new_text: &str, model: &gtk::TreeStore) {
    if let Some(iter) = model.get_iter(&path) {
        let old_value = model.get_value(&iter, 2);
        debug!("{:?}", old_value.get::<String>());
        model.set_value(&iter, 2, &new_text.to_value());
    }
}
