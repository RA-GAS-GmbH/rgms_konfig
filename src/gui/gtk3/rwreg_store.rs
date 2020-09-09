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
                    &reg.protected(),
                ],
            );
        }
    }
}
