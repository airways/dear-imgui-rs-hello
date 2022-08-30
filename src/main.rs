// Note: this example is currently based on the `imgui` crate
// 0.8.2. Version 0.9.0 is a significant refactor currently in
// preview state, a corresponding version of this repo is
// available in the branch release/0.9.0
//
// Note: this example is not currently working with Window's
// text scaling options. This should be fixed with the release
// of `imgui` crate version 0.9.0, which I believe has fixed
// this issue.

use std::fmt::Write;
use std::collections::HashMap;
use std::time::Instant;
use imgui::*;

mod support;

// Used to remember user actions during program execution
struct State {
    // General initialization flag, true when UI is fully ready.
    // False during loading screen.
    ready: bool,
    
    // Loading progress as fraction (range 0.0f..1.0f)
    loading_percentage: f32,

    // Main window adds, gets and deletes keys from this map
    // which acts as a dummy store, values are lost on exit.
    pairs: HashMap<String, String>,

    // List of fake documents to render separate windows for
    docs: Vec<String>,

    // UI strings mapped to input fields
    key: String,
    value: String,
    output: String,

    // Flags to show/hide main non-popup windows
    show_main_window: bool,
    show_style_editor: bool,
    show_window_2: bool,

    // Menu state structs
    file_menu: FileMenuState,
}

impl Default for State {
    fn default() -> Self {
        // Initialize dummy "data store", a simple String to String
        // hash map for keys and another one for "documents".
        let pairs = HashMap::new();
        let docs = Vec::new();

        // Initial text input values to some useful defaults
        let key = String::from("message");
        let value = String::from("This is a test value");

        // Empty log output with a preallocated buffer
        let output: String = String::with_capacity(2048);
        
        State {
            // Loading
            ready: false,
            loading_percentage: 0.0,
            // Data
            pairs,
            docs,
            // UI
            key,
            value,
            output,
            // Window flags
            show_main_window: true,
            show_style_editor: false,
            show_window_2: false,
            // Menu state
            file_menu: Default::default(),
        }
    }
}

struct FileMenuState {
    enabled: bool,
    f: f32,
    n: usize,
    b: bool,
}

impl Default for FileMenuState {
    fn default() -> Self {
        FileMenuState {
            enabled: true,
            f: 0.5,
            n: 0,
            b: true,
        }
    }
}

fn show_example_menu_file<'a>(ui: &Ui<'a>, state: &mut State) {
    let menu_state = &mut state.file_menu;

    if MenuItem::new("New").build(ui) {
        state.docs.insert(0, format!("New Document {}", state.docs.len() + 1));
    }
    
    if MenuItem::new("Toggle Style Editor").build(ui) {
        state.show_style_editor = !state.show_style_editor;
    }

    ui.separator();
    MenuItem::new("(items below are dummies)").enabled(false).build(ui);
    ui.separator();

    MenuItem::new("Open").shortcut("Ctrl+O").build(ui);
    if let Some(menu) = ui.begin_menu("Open Recent") {
        MenuItem::new("fish_hat.c").build(ui);
        MenuItem::new("fish_hat.inl").build(ui);
        MenuItem::new("fish_hat.h").build(ui);
        if let Some(menu) = ui.begin_menu("More..") {
            MenuItem::new("Hello").build(ui);
            MenuItem::new("Sailor").build(ui);
            menu.end();
        }
        menu.end();
    }
    MenuItem::new("Save").shortcut("Ctrl+S").build(ui);
    MenuItem::new("Save As..").build(ui);
    ui.separator();
    if let Some(menu) = ui.begin_menu("Options") {
        MenuItem::new("Enabled").build_with_ref(ui, &mut menu_state.enabled);
        ChildWindow::new("child")
            .size([0.0, 60.0])
            .border(true)
            .build(ui, || {
                for i in 0..10 {
                    ui.text(format!("Scrolling Text {}", i));
                }
            });
        Slider::new("Value", 0.0, 1.0).build(ui, &mut menu_state.f);

        ui.input_float("Input", &mut menu_state.f).step(0.1).build();
        let items = ["Yes", "No", "Maybe"];
        ui.combo_simple_string("Combo", &mut menu_state.n, &items);
        ui.checkbox("Check", &mut menu_state.b);
        menu.end();
    }
    if let Some(menu) = ui.begin_menu("Colors") {
        for &col in StyleColor::VARIANTS.iter() {
            MenuItem::new(format!("{:?}", col)).build(ui);
        }
        menu.end();
    }
    assert!(ui.begin_menu_with_enabled("Disabled", false).is_none());
    MenuItem::new("Checked").selected(true).build(ui);
    MenuItem::new("Quit").shortcut("Alt+F4").build(ui);
}

fn main() {
    // Default OS window size
    let frame_w = 800f64;
    let frame_h = 600f64;

    let mut system = support::init("dear imgui-rs, hello!", frame_w, frame_h,
    support::SystemColor { red: 0.03, green: 0.07, blue: 0.04, alpha: 0.94 });
    
    let mut state = State::default();
    
    // Used to fake loading
    let now = Instant::now();
    let style = system.imgui.style_mut();

    // inner windows
    style.window_border_size = 1.0;
    style.window_rounding = 4.0;
    // controls
    style.frame_rounding = 4.0;
    // non-modal popups (used for confirmation in this example)
    style.popup_border_size = 1.0;
    style.popup_rounding = 4.0;

    // theme colors -- "blue hydrangea"
    // you can easily generate these values via the built in 
    // file > Toggle Style Editor command in this example.
    // there are also many in this issue on the C++ imgui
    // repo: https://github.com/ocornut/imgui/issues/707
    style.colors[StyleColor::Text as usize] = [1.00, 1.00, 1.00, 1.00];
    style.colors[StyleColor::TextDisabled as usize] = [0.50, 0.50, 0.50, 1.00];
    style.colors[StyleColor::WindowBg as usize] = [0.03, 0.07, 0.04, 0.94];
    style.colors[StyleColor::ChildBg as usize] = [0.00, 0.00, 0.00, 0.00];
    style.colors[StyleColor::PopupBg as usize] = [0.08, 0.08, 0.08, 0.94];
    style.colors[StyleColor::Border as usize] = [0.38, 1.00, 0.00, 0.50];
    style.colors[StyleColor::BorderShadow as usize] = [0.01, 0.13, 0.00, 0.63];
    style.colors[StyleColor::FrameBg as usize] = [0.17, 0.48, 0.16, 0.54];
    style.colors[StyleColor::FrameBgHovered as usize] = [0.26, 0.98, 0.32, 0.40];
    style.colors[StyleColor::FrameBgActive as usize] = [0.26, 0.98, 0.28, 0.67];
    style.colors[StyleColor::TitleBg as usize] = [0.01, 0.07, 0.01, 1.00];
    style.colors[StyleColor::TitleBgActive as usize] = [0.0, 0.29, 0.68, 1.0];
    style.colors[StyleColor::TitleBgCollapsed as usize] = [0.00, 0.56, 0.09, 0.51];
    style.colors[StyleColor::MenuBarBg as usize] = [0.0, 0.29, 0.68, 1.0];
    style.colors[StyleColor::ScrollbarBg as usize] = [0.00, 0.15, 0.00, 0.53];
    style.colors[StyleColor::ScrollbarGrab as usize] = [0.10, 0.41, 0.06, 1.00];
    style.colors[StyleColor::ScrollbarGrabHovered as usize] = [0.00, 0.66, 0.04, 1.00];
    style.colors[StyleColor::ScrollbarGrabActive as usize] = [0.04, 0.87, 0.00, 1.00];
    style.colors[StyleColor::CheckMark as usize] = [0.26, 0.98, 0.40, 1.00];
    style.colors[StyleColor::SliderGrab as usize] = [0.21, 0.61, 0.00, 1.00];
    style.colors[StyleColor::SliderGrabActive as usize] = [0.36, 0.87, 0.22, 1.00];
    style.colors[StyleColor::Button as usize] = [0.00, 0.60, 0.05, 0.40];
    style.colors[StyleColor::ButtonHovered as usize] = [0.20, 0.78, 0.32, 1.00];
    style.colors[StyleColor::ButtonActive as usize] = [0.00, 0.57, 0.07, 1.00];
    style.colors[StyleColor::Header as usize] = [0.12, 0.82, 0.28, 0.31];
    style.colors[StyleColor::HeaderHovered as usize] = [0.00, 0.74, 0.11, 0.80];
    style.colors[StyleColor::HeaderActive as usize] = [0.09, 0.69, 0.04, 1.00];
    style.colors[StyleColor::Separator as usize] = [0.09, 0.67, 0.01, 0.50];
    style.colors[StyleColor::SeparatorHovered as usize] = [0.32, 0.75, 0.10, 0.78];
    style.colors[StyleColor::SeparatorActive as usize] = [0.10, 0.75, 0.11, 1.00];
    style.colors[StyleColor::ResizeGrip as usize] = [0.32, 0.98, 0.26, 0.20];
    style.colors[StyleColor::ResizeGripHovered as usize] = [0.26, 0.98, 0.28, 0.67];
    style.colors[StyleColor::ResizeGripActive as usize] = [0.22, 0.69, 0.06, 0.95];
    style.colors[StyleColor::Tab as usize] = [0.18, 0.58, 0.18, 0.86];
    style.colors[StyleColor::TabHovered as usize] = [0.26, 0.98, 0.28, 0.80];
    style.colors[StyleColor::TabActive as usize] = [0.20, 0.68, 0.24, 1.00];
    style.colors[StyleColor::TabUnfocused as usize] = [0.07, 0.15, 0.08, 0.97];
    style.colors[StyleColor::TabUnfocusedActive as usize] = [0.14, 0.42, 0.19, 1.00];
    style.colors[StyleColor::PlotLines as usize] = [0.61, 0.61, 0.61, 1.00];
    style.colors[StyleColor::PlotLinesHovered as usize] = [1.00, 0.43, 0.35, 1.00];
    style.colors[StyleColor::PlotHistogram as usize] = [0.90, 0.70, 0.00, 1.00];
    style.colors[StyleColor::PlotHistogramHovered as usize] = [1.00, 0.60, 0.00, 1.00];
    style.colors[StyleColor::TableHeaderBg as usize] = [0.19, 0.19, 0.20, 1.00];
    style.colors[StyleColor::TableBorderStrong as usize] = [0.31, 0.31, 0.35, 1.00];
    style.colors[StyleColor::TableBorderLight as usize] = [0.23, 0.23, 0.25, 1.00];
    style.colors[StyleColor::TableRowBg as usize] = [0.00, 0.00, 0.00, 0.00];
    style.colors[StyleColor::TableRowBgAlt as usize] = [1.00, 1.00, 1.00, 0.06];
    style.colors[StyleColor::TextSelectedBg as usize] = [0.00, 0.89, 0.20, 0.35];
    style.colors[StyleColor::DragDropTarget as usize] = [1.00, 1.00, 0.00, 0.90];
    style.colors[StyleColor::NavHighlight as usize] = [0.26, 0.98, 0.35, 1.00];
    style.colors[StyleColor::NavWindowingHighlight as usize] = [1.00, 1.00, 1.00, 0.70];
    style.colors[StyleColor::NavWindowingDimBg as usize] = [0.80, 0.80, 0.80, 0.20];
    style.colors[StyleColor::ModalWindowDimBg as usize] = [0.80, 0.80, 0.80, 0.35];

    system.main_loop(move |_, ui| {
        if !state.ready {
            if state.loading_percentage >= 1.0 {
                ui.close_current_popup();
                state.ready = true
            } else {
                // Simulate loading progress that takes 2 seconds
                state.loading_percentage = now.elapsed().as_millis() as f32 / 2000.0;
                ui.open_popup("Loading");
                ui.popup("Loading", || {
                    ui.text("L O A D I N G . . .");
                    ProgressBar::new(state.loading_percentage).build(ui);
                });
            }
        } else {
            if let Some(menu_bar) = ui.begin_main_menu_bar() {
                if let Some(menu) = ui.begin_menu("File") {
                    show_example_menu_file(ui, &mut state);
                    menu.end();
                }
                if let Some(menu) = ui.begin_menu("Edit") {
                    MenuItem::new("Undo").shortcut("CTRL+Z").build(ui);
                    MenuItem::new("Redo")
                        .shortcut("CTRL+Y")
                        .enabled(false)
                        .build(ui);
                    ui.separator();
                    MenuItem::new("Cut").shortcut("CTRL+X").build(ui);
                    MenuItem::new("Copy").shortcut("CTRL+C").build(ui);
                    MenuItem::new("Paste").shortcut("CTRL+V").build(ui);
                    menu.end();
                }
                menu_bar.end();
            }

            if state.show_style_editor {
                //ui.show_default_style_editor();
                Window::new("Active Stle")
                .position([15.0, 25.0], Condition::FirstUseEver)
                .size([450.0, 500.0], Condition::FirstUseEver)
                .build(ui, || {
                    ui.show_style_editor(&mut ui.clone_style());
                });
            }

            // Main window
            if state.show_main_window {
                Window::new(file!())
                .position([15.0, 25.0], Condition::FirstUseEver)
                .size([450.0, 500.0], Condition::FirstUseEver)
                .build(ui, || {
                    let start_ouput_len = state.output.len();

                    if !state.ready {
                        _ = write!(state.output, "     ~~~ dear imgui-rs, hello! ~~~     \n\n");
                        _ = write!(state.output, "Ready.\n");
                        state.ready = true;
                    }

                    ui.input_text("Key", &mut state.key).build();
                    ui.input_text("Value", &mut state.value).build();

                    if ui.button("Set") {
                        if state.value.len() == 0 {
                            _ = write!(state.output, "Value for `{}` is blank, not setting.\nUse the [Del] button to remove a key.\n", &state.key);
                        } else {
                            if state.pairs.contains_key(&state.key) {
                                _ = write!(state.output, "Key {} exists, removing old value.\n", &state.key);
                            }
                            _ = write!(state.output, "Setting key `{}` = `{}`\n", &state.key, &state.value);
                            state.pairs.insert(state.key.to_owned(), state.value.to_owned());
                            // So we can easily see that the get works, clear the value
                            state.value = String::new();
                        }
                        dbg!(&state.pairs);
                    }

                    ui.same_line();
                    if ui.button("Get") {
                        if state.pairs.contains_key(&state.key) {
                            let val = state.pairs.get(&state.key);
                            if val.is_some() {
                                state.value = val.unwrap().to_owned();
                                _ = write!(state.output, "Got `{}`s value: `{}`\n", &state.key, &state.value);
                            } else {
                                _ = write!(state.output, "Key `{}` appears to exist, but the value is not valid.\n", &state.key);
                            }
                        } else {
                            _ = write!(state.output, "Key `{}` is not set!\n", &state.key);
                        }
                        dbg!(&state.pairs);
                    }

                    ui.same_line();
                    if ui.button("Del") {
                        if state.key.len() != 0 {
                            if state.pairs.contains_key(&state.key) {
                                //state.modal_delete = true;
                                ui.open_popup("Delete?");
                            } else {
                                _ = write!(state.output, "Key `{}` is not set!\n", &state.key);
                            }
                        }
                        dbg!(&state.pairs);
                    }

                    // Floating confirmation popup for Del button
                    // Clicking outside the popup will also cancel it
                    ui.popup("Delete?", || {
                        ui.text(format!("Are you sure you want to delete this key?\n\n\t`{}`\n ", &state.key));
                        ui.separator();
                        if ui.button("Delete Key") {
                            _ = write!(state.output, "Removing key `{}`!\n", &state.key);
                            state.pairs.remove(&state.key);
                            dbg!(&state.pairs);
                            ui.close_current_popup();
                        }
                        ui.same_line();
                        if ui.button("Cancel") {
                            ui.close_current_popup();
                        }
                    });
                    

                    ChildWindow::new("output")
                        .size([425.0, 300.0])
                        .always_vertical_scrollbar(true)
                        .build(ui, || {
                            ui.text(format!(
                                "Output:\n{}",
                                &state.output
                            ));
                            if state.output.len() > start_ouput_len {
                                ui.set_scroll_y(ui.scroll_y()+ui.frame_height_with_spacing()*2.0);
                            }
                        });
                    
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));

                    if ui.button("Toggle Window #2") {
                        state.show_window_2 = !state.show_window_2;
                        if state.show_window_2 {
                            _ = write!(state.output, "Showing window!\n");
                        } else {
                            _ = write!(state.output, "Hiding window!\n");
                        }
                    }
                });
            }

            // Other windows
            if state.show_window_2 {
                Window::new(format!("Another window!"))
                    .position([500.0, 50.0], Condition::FirstUseEver)
                    .size([200.0, 200.0], Condition::FirstUseEver)
                    .build(ui, || {
                        
                    ui.text("Hello from another window!");
                    ui.separator();
                    ui.text("It is quite exiting!");

                });
            }

            // Document windows
            let mut doc_count = 0;
            for title in &state.docs {
                Window::new(format!("Document: {}", title))
                    .position([10.0 + 25.0 * doc_count as f32, 25.0 + 25.0 * doc_count as f32], 
                        Condition::FirstUseEver)
                    .size([200.0, 200.0], Condition::FirstUseEver)
                    .build(ui, || {
                        
                    });
                doc_count += 1;
            }
        }
    });
    
}
