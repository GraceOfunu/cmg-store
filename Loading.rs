use druid::widget::{Container, Flex, Image, Label};
use druid::Color;
use druid::{AppLauncher, Data, Lens, Widget, WindowDesc};
use std::time::Duration;

// Define the data structure for the loading page
#[derive(Clone, Data, Lens)]
struct LoadingPageData {
    progress: f64,
}

impl Default for LoadingPageData {
    fn default() -> Self {
        LoadingPageData { progress: 0.0 }
    }
}

// Create the widget for the loading page
fn loading_page() -> impl Widget<LoadingPageData> {
    // Flex is used to lay out the widgets horizontally or vertically
    Flex::column()
        .with_flex_child(
            // Background container
            Container::new(
                // Gradient color for the background
                Flex::row()
                    .with_flex_child(
                        Container::new(()).background(Color::linear_gradient(
                            Color::rgb8(15, 195, 174),
                            Color::rgba8(15, 195, 174, 0),
                            1024.0,
                        )),
                        44.0,
                    )
                    .with_flex_child(Container::new(()), 1.0)
                    .background(Color::WHITE),
            )
            .expand_height(),
            1.0,
        )
        .with_flex_child(
            // Progress bar container
            Container::new(
                Flex::row()
                    .with_child(
                        // Progress bar background
                        Container::new(())
                            .background(Color::linear_gradient(
                                Color::rgba8(255, 255, 255, 0.60),
                                Color::rgba8(255, 255, 255, 0),
                                34.0,
                            ))
                            .expand_width()
                            .expand_height(),
                    )
                    .with_child(
                        // Progress bar
                        Container::new(())
                            .background(Color::WHITE)
                            .width(171.0 * LoadingPageData::get().progress), // Adjust progress width
                    ),
            )
            .expand_width()
            .height(34.0)
            .padding((0.0, 777.0, 0.0, 0.0)),
            0.0,
        )
        .with_flex_child(
            // Centered content
            Flex::row()
                .main_axis_alignment(druid::widget::MainAxisAlignment::Center)
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Center)
                .with_flex_child(
                    // The logo image
                    Image::new(druid::ImageBuf::empty().into()), // Replace ImageBuf::empty() with the actual image data
                    1.0,
                )
                .with_child(
                    // Text label
                    Label::new("CMG")
                        .with_text_color(Color::WHITE)
                        .with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE).with_size(300.0))
                        .with_text_alignment(druid::TextAlignment::Center)
                        .fix_height(100.0) // Adjust height as needed
                ),
            1.0,
        )
}

fn main() {
    // Set up the main window with the loading page
    let main_window = WindowDesc::new(loading_page)
        .window_size((1440.0, 1024.0))
        .resizable(false);

    // Launch the app with the loading page data
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(LoadingPageData::default())
        .expect("Failed to launch the app.");

    // Simulate progress animation
    let mut progress = 0.0;
    while progress <= 1.0 {
        std::thread::sleep(Duration::from_millis(100));
        progress += 0.05; // Change the step value to control the animation speed
        if let Some(app_state) = AppLauncher::get_current().map(|launcher| launcher.get_app_state()) {
            app_state.submit_command(druid::commands::TIMER_UPDATE.with(progress)).ok();
        }
    }
}
