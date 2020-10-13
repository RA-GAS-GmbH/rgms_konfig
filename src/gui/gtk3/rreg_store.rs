use crate::platine::BoxedPlatine;
use gtk::prelude::*;
use std::sync::{Arc, Mutex};

/// Resource counted, clonbarer, optionaler TreeStore
///
/// In diesem Typen wird der TreeStore der Lese-Register gespeichert.
pub type BoxedRregStore = Arc<Mutex<Option<RregStore>>>;

/// GtkTreestore for a Rwreg
pub struct RregStore {
    store: gtk::TreeStore,
    platine: BoxedPlatine,
}

impl RregStore {
    /// Erstellt eine neuen RregStore
    pub fn new(platine: BoxedPlatine) -> Self {
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

        RregStore {
            store,
            platine,
        }
    }

    /// Füllt den TreeStore mit Daten
    fn fill_treestore(&self) {
        if let Ok(p) = self.platine.lock() {
            if let Some(platine) = &*p {
                for reg in platine.rregs() {
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
        }
    }

    /// Füllt den TreeStore mit Daten und buildet die GUI Komponenten
    pub fn fill_and_build_ui(&self) -> gtk::ScrolledWindow {
        self.fill_treestore();
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
    pub fn update_treestore(&self, _new_values: &[u16]) {
        // if let Some(iter) = self.store.get_iter_first() {
        //     let _: Vec<u16> = new_values
        //         .iter()
        //         .enumerate()
        //         .map(|(i, value)| {
        //             let reg_nr = self
        //                 .store
        //                 .get_value(&iter, 0)
        //                 .get::<u32>()
        //                 .unwrap_or(Some(0))
        //                 .unwrap_or(0);
        //             debug!("i: {} reg_nr: {} value: {}", i, reg_nr, value);
        //             if i as u32 == reg_nr {
        //                 let val = (*value as u32).to_value();
        //                 self.store.set_value(&iter, 2, &val);
        //                 self.store.iter_next(&iter);
        //             }
        //             *value
        //         })
        //         .collect();
    }
}
