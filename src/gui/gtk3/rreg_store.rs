use crate::platine::Platine;
use gtk::prelude::TreeStoreExtManual;

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
    fn fill_treestore(&self, platine: Box<dyn Platine>) {
        for reg in platine.rregs() {
            self.store.insert_with_values(
                None,
                None,
                &[0, 1, 2, 3],
                &[
                    &reg.rreg_nr(),
                    &reg.range(),
                    &reg.value(),
                    &reg.description(),
                ],
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "gtk::test not working"]
    fn fill_treestore() {
        let _store = RregStore::new();
    }
}
