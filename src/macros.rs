
/// Simplify implementation of BoilerPlate widget module functions.
macro_rules! widget_fns(
    ($widget:ident, $widget_state:ident, $default:expr) => (

        /// Default Widget variant.
        fn default() -> ::widget::Widget { $default }

        /// Get a reference to the widget associated with the given UIID.
        fn get_widget<C>(
            ui: &mut ::ui::Ui<C>,
            ui_id: ::ui::UIID
        ) -> &mut ::widget::Widget {
            ui.get_widget(ui_id, default())
        }

        /// Get the current State for the widget.
        fn get_state<C>(
            ui: &mut ::ui::Ui<C>,
            ui_id: ::ui::UIID
        ) -> &$widget_state {
            match *get_widget(ui, ui_id) {
                ::widget::Widget::$widget(ref state) => state,
                _ => panic!("The Widget variant returned by uiontext is different to that which \
                           was requested (Check that there are no UIID conflicts)."),
            }
        }

        /// Set the state for the widget in the Ui.
        fn set_state<C>(
            ui: &mut ::ui::Ui<C>,
            ui_id: ::ui::UIID,
            new_state: ::widget::Widget,
            pos: ::point::Point,
            dim: ::dimensions::Dimensions
        ) {
            match *get_widget(ui, ui_id) {
                ref mut state => {
                    if !state.matches(&new_state) {
                        panic!("The Widget variant returned by Ui is different to that which \
                                   was requested (Check that there are no UIID conflicts).");
                    }
                    *state = new_state;
                }
            }
            ui.set_place(ui_id, pos, dim);
        }

    )
);
