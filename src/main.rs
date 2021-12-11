use cursive::{
    traits::Boxable,
    views::{Button, CircularFocus, Dialog, DummyView, LinearLayout, Panel, ProgressBar, TextView},
    Cursive, CursiveExt, With,
};
use rppal::{
    gpio::Gpio,
    pwm::{Channel, Polarity, Pwm},
};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpio = Gpio::new()?;

    let out1 = Arc::new(Mutex::new(gpio.get(5)?.into_output_low()));
    let out1_clone1 = out1.clone();
    let out1_clone2 = out1.clone();
    let out2 = Arc::new(Mutex::new(gpio.get(6)?.into_output_low()));
    let out2_clone1 = out2.clone();
    let out2_clone2 = out2.clone();

    let pwm = Pwm::with_frequency(Channel::Pwm0, 1000.0, 0.0, Polarity::Normal, true)?;
    let pwm = Arc::new(Mutex::new(pwm));
    let pwm_clone = pwm.clone();

    // for i in 1..10 {
    //     pwm.set_duty_cycle(i as f64 * 0.1)?;

    //     out2.set_high();
    //     thread::sleep(Duration::from_secs(2));

    //     out2.set_low();
    //     thread::sleep(Duration::from_secs(1));

    //     out1.set_high();
    //     thread::sleep(Duration::from_secs(2));

    //     out1.set_low();
    //     thread::sleep(Duration::from_secs(1));
    // }
    let mut siv = Cursive::new();
    let speed = Arc::new(Mutex::new(0));
    let speed_clone1 = speed.clone();
    let speed_clone2 = speed.clone();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(
        Panel::new(
            LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                        .child(
                            Dialog::around(TextView::new(""))
                                .title("Direction")
                                .button(" <- ", move |_| {
                                    let mut out1 =
                                        out1_clone1.lock().expect("out1 pin lock failed");
                                    let mut out2 =
                                        out2_clone1.lock().expect("out2 pin lock failed");

                                    out1.set_low();
                                    out2.set_high();
                                })
                                .button(" -> ", move |_| {
                                    let mut out1 =
                                        out1_clone2.lock().expect("out1 pin lock failed");
                                    let mut out2 =
                                        out2_clone2.lock().expect("out2 pin lock failed");

                                    out1.set_high();
                                    out2.set_low();
                                })
                                .wrap_with(|v| CircularFocus::new(v, true, true)), // .fixed_width(30),
                        )
                        .child(DummyView.fixed_width(1))
                        .child(
                            Dialog::around(TextView::new(""))
                                .title("Speed")
                                .button(" + ", move |_| {
                                    let mut speed = speed.lock().unwrap();

                                    if *speed < 100 {
                                        *speed += 1;
                                    }

                                    let pwm = pwm_clone.lock().expect("pwm lock failed");

                                    pwm.set_duty_cycle(*speed as f64 * 0.01).ok();
                                })
                                .button(" - ", move |_| {
                                    let mut speed = speed_clone1.lock().unwrap();

                                    if *speed > 0 {
                                        *speed -= 1;
                                    }

                                    let pwm = pwm.lock().expect("pwm lock failed");

                                    pwm.set_duty_cycle(*speed as f64 * 0.01).ok();
                                })
                                .wrap_with(|v| CircularFocus::new(v, true, true)), // .fixed_width(30),
                        ),
                )
                .child(DummyView.fixed_height(1))
                .child(Button::new("Stop", move |_| {
                    let mut out1 = out1.lock().expect("out1 pin lock failed");
                    let mut out2 = out2.lock().expect("out2 pin lock failed");

                    out1.set_low();
                    out2.set_low();
                }))
                .child(DummyView.fixed_height(1))
                .child(ProgressBar::default().with_task(move |counter| loop {
                    let i = speed_clone2.lock().expect("Mutex speed lock failed");

                    counter.set(*i);
                })),
        )
        .title("DC Motor Controller"),
    );

    // siv.add_layer(
    //     // Most views can be configured in a chainable way
    //     Dialog::around(TextView::new("Choose speed"))
    //         .title("Speed")
    //         .button(" + ", |s| ())
    //         .button(" - ", |s| s.quit())
    //         .wrap_with(|v| CircularFocus::new(v, true, true)),
    // );
    siv.add_global_callback('q', |s| s.quit());

    siv.run();

    Ok(())
}
