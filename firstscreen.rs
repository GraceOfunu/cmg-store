use druid::widget::{Container, Flex, Image, Label};
use druid::Color;
use druid::{AppLauncher, Data, Lens, Widget, WindowDesc};

// Define the data structure for the welcome screen
#[derive(Clone, Data, Lens)]
struct WelcomeScreenData {
    title: String,
}

// Create the widget for the welcome screen
fn welcome_screen() -> impl Widget<WelcomeScreenData> {
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
            // Centered content
            Flex::row()
                .main_axis_alignment(druid::widget::MainAxisAlignment::Center)
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::Center)
                .with_flex_child(
                    // The logo image
                    Image::new(druid::ImageBuf::empty().into()) // Replace ImageBuf::empty() with the actual image data
                        .width(351.0)
                        .height(354.0),
                    1.0,
                )
                .with_child(
                    // Text label
                    Label::new(|data: &WelcomeScreenData, _env: &druid::Env| data.title.clone())
                        .with_text_color(Color::WHITE)
                        .with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE).with_size(300.0)),
                ),
            1.0,
        )
}

fn main() {
    // Create the initial data for the welcome screen
    let welcome_data = WelcomeScreenData {
        title: "Welcome to CMG".to_string(),
    };

    // Set up the main window with the welcome screen
    let main_window = WindowDesc::new(welcome_screen)
        .window_size((1440.0, 1024.0))
        .resizable(false);

    // Launch the app with the welcome screen data
    AppLauncher::with_window(main_window)
        .launch(welcome_data)
        .expect("Failed to launch the app.");
}
