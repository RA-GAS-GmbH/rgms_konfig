use crate::platine::BoxedPlatine;
use gtk::prelude::*;
use std::sync::{Arc, Mutex};
use glib::clone;

/// Resource counted, clonbarer, optionaler TreeStore
///
/// In diesem Typen wird der TreeStore der Schreib.-/ Lese-Register gespeichert.
pub type BoxedRwregStore = Arc<Mutex<Option<RwregStore>>>;

/// GtkTreestore for a Rwreg
pub struct RwregStore {
    store: gtk::TreeStore,
    platine: BoxedPlatine,
}

impl RwregStore {
    /// Erstellt eine neuen RwregStore
    pub fn new(platine: BoxedPlatine) -> Self {
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

        RwregStore { store, platine }
    }

    /// Füllt den TreeStore mit Daten
    fn fill_treestore(&self) {
        if let Ok(p) = self.platine.lock() {
            if let Some(platine) = &*p {
                for reg in platine.rwregs() {
                    self.store.insert_with_values(
                        None,
                        None,
                        &[0, 1, 2, 3, 4],
                        &[
                            &reg.reg_nr(),
                            &reg.range(),
                            &reg.value(),
                            &reg.description(),
                            &reg.is_protected(),
                        ],
                    );
                }
            }
        }
    }

    /// Füllt den TreeStore mit Daten und buildet die GUI Komponenten
    pub fn fill_and_build_ui(
        &self,
        gui_tx: &futures::channel::mpsc::Sender<crate::gui::gtk3::GuiMessage>,
        // modbus_master_tx: &tokio::sync::mpsc::Sender<crate::modbus_master::ModbusMasterMessage>,
    ) -> gtk::ScrolledWindow {
        self.fill_treestore();
        let sortable_store = gtk::TreeModelSort::new(&self.store);
        let treeview = gtk::TreeView::with_model(&sortable_store);

        treeview.set_grid_lines(gtk::TreeViewGridLines::Horizontal);

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
        renderer.set_alignment(0.5, 1.0);
        renderer.set_property_editable(true);
        column_value.pack_end(&renderer, true);
        column_value.add_attribute(&renderer, "text", 2);
        treeview.append_column(&column_value);
        // Callbacks
        let store = self.store.clone();

        renderer.connect_edited(clone!(
            @strong gui_tx
            => move |_widget, path, text| {
            // debug!("Edited:\nwidget: {:?}\npath: {:?}\ntext: {:?}\n", widget, path, text);
            callback_edit_cell(&path, text, &store, &gui_tx);
        }));

        // Renderer Column 3
        let column_property = gtk::TreeViewColumn::new();
        column_property.set_title("Messwerteigenschaft");
        let renderer = gtk::CellRendererText::new();
        column_property.pack_end(&renderer, true);
        column_property.add_attribute(&renderer, "text", 3);
        treeview.append_column(&column_property);

        // 2020-09-15 20:10:18 smueller: Auskommentiert weil es nicht gut aussieht
        //
        // // Renderer Column 4
        // let column_property = gtk::TreeViewColumn::new();
        // column_property.set_title("geschützt");
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
    pub fn update_treestore(&self, values: Vec<(u16, u16)>) {
        if let Some(iter) = self.store.get_iter_first() {
            for (_reg_nr, value) in values {
                self.store.set_value(&iter, 2, &(value as u32).to_value());
                self.store.iter_next(&iter);
            }
        }
    }
}

/// Callback wird ausgeführt wenn ein Benutzer einen Wert in der Rwreg Liste aktualisiert
///
/// Da der Callback nicht im Gui Thread bearbeitet wird stehen die GUI Komponenten nicht
/// so einfach zur Verfügung.
/// Desshalb wird aus dem Callback eine Nachricht an den GUI Thread gesendet. Die Logik
/// sieht in etwa so aus:
/// Rwreg Callback -> GuiMessage -> ModbusMasterMessage -> GuiMessage
fn callback_edit_cell(
    path: &gtk::TreePath,
    new_value_text: &str,
    model: &gtk::TreeStore,
    gui_tx: &futures::channel::mpsc::Sender<crate::gui::gtk3::GuiMessage>,
) {
    if let Some(iter) = model.get_iter(&path) {
        let reg_nr = model.get_value(&iter, 0);
        let reg_nr = reg_nr.get_some::<u32>()
            .map_err(|_error| {})
            .map(|reg_nr| { reg_nr as u16 })
            .map_err(|_error| {});

        // let old_value = model.get_value(&iter, 2);
        // let old_value = old_value.get_some::<u32>()
        //     .map_err(|_error| {})
        //     .map(|old_value| { old_value as u16 })
        //     .map_err(|_error| {});

        let new_value = new_value_text.to_string();
        // let modbus_master_tx = modbus_master_tx.clone();

        match gui_tx.clone().try_send(crate::gui::gtk3::GuiMessage::ModbusMasterUpdateRegister {reg_nr, new_value}) {
            Ok(_) => {
                model.set_value(&iter, 2, &new_value_text.to_value());
            }
            Err(_error) => {}
        }
    }
}
