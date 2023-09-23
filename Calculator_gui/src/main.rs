use fltk::{
    app::*, button::*, enums::FrameType, frame::*, prelude::*, window::*, enums::Color,
};
use std::cell::RefCell;
use std::rc::Rc;
use eval::eval;

fn main() {
    // Create the FLTK application
    let app = App::default();

    // Create the main window
    let mut wind = Window::new(100, 100, 340, 480, "Calculator");

    // Create the frame to display the calculator output
    let mut frame = Frame::new(10, 30, 320, 100, "");
    wind.set_color(Color::from_rgb(32, 32, 32));
    frame.set_frame(FrameType::FlatBox);
    frame.set_label_size(24);
    frame.set_color(Color::from_rgb(59 , 59 , 59));
    frame.set_label_color(Color::White);

    // Create a shared reference-counted string to store the number being entered
    let number = Rc::new(RefCell::new(String::new()));

    // Create a shared reference-counted frame to be used in button event handlers
    let frame = Rc::new(RefCell::new(frame));

    // Create the buttons
    let mut buttons = Vec::<Button>::new();
    let button_labels = [
        "sin", "cos", "tan", "CE",
        "7", "8", "9", "/",
        "4", "5", "6", "*",
        "1", "2", "3", "-",
        ".", "0", "+", "=",
    ];

    for (i, label) in button_labels.iter().enumerate() {
        let mut button = Button::new(
            (10 + (i % 4) * 81).try_into().unwrap(),
            (160 + (i / 4) * 61).try_into().unwrap(),
            80,
            60,
            &**label,
        );
        button.set_color(Color::from_rgb(50, 50, 50));
        button.set_label_color(Color::White);
        button.set_label_size(20);
        buttons.push(button);
    }

    // Set up event handlers for each button
    for button in &mut buttons {
        let clone_number = Rc::clone(&number);
        let clone_frame = Rc::clone(&frame);

        button.handle(move |button: &mut Button, event: fltk::enums::Event| {
            let mut number = clone_number.borrow_mut();
            let mut frame = clone_frame.borrow_mut();
            let label = button.label();

            if event == fltk::enums::Event::Push && label == "=" {
                // Evaluate the expression and display the result in the frame
                let result = evaluate(&number);
                frame.set_label(&result);
                number.clear();
            } else if event == fltk::enums::Event::Push && label == "CE" {
                // Clear the number and the frame
                number.clear();
                frame.set_label("");
            } else if event == fltk::enums::Event::Push && label == "sin"{
                // Calculate the sine of the number and display the result in the frame
                let angle: f64 = number.parse().unwrap();
                let radians_angle = angle.to_radians();
                let sin_result = radians_angle.sin();
                let sin_text = sin_result.to_string();
                let result = evaluate(&sin_text);
                frame.set_label(&result);
                number.clear();
            } else if event == fltk::enums::Event::Push && label == "cos"{
                // Calculate the cosine of the number and display the result in the frame
                let angle: f64 = number.parse().unwrap();
                let radians_angle = angle.to_radians();
                let cos_result = radians_angle.cos();
                let cos_text = cos_result.to_string();
                let result = evaluate(&cos_text);
                frame.set_label(&result);
                number.clear();
            } else if event == fltk::enums::Event::Push && label == "tan"{
                // Calculate the tangent of the number and display the result in the frame
                let angle: f64 = number.parse().unwrap();
                let radians_angle = angle.to_radians();
                let tan_result = radians_angle.tan();
                let tan_text = tan_result.to_string();
                let result = evaluate(&tan_text);
                frame.set_label(&result);
                number.clear();
            } else if event == fltk::enums::Event::Push {
                // Append the button label to the number and update the frame
                number.push_str(&label);
                frame.set_label(&number);
            }
            true
        });
    }

    wind.end();
    wind.show();
    app.run().unwrap();
}

// Evaluate the expression using the 'eval' library
fn evaluate(expression: &str) -> String {
    match eval(expression) {
        Ok(result) => result.to_string(),
        Err(error) => format!("Error: {}", error),
    }
}