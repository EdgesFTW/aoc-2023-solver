use aoc2023::{
    day1::{day1_pt1, day1_pt2},
    day2::{day2_pt1, day2_pt2},
    day3::{day3_pt1, day3_pt2},
    day4::{day4_pt1, day4_pt2},
};
use leptos::{
    component, create_effect, create_node_ref, create_signal,
    html::{Select, Textarea},
    leptos_dom::logging::console_log,
    view, IntoView,
};

fn aoc_func(day: &str, part: &str) -> Option<Box<dyn Fn(String) -> i64>> {
    match day {
        "day1" => match part {
            "pt1" => Some(Box::new(day1_pt1)),
            "pt2" => Some(Box::new(day1_pt2)),
            _ => Some(Box::new(day1_pt1)),
        },
        "day2" => match part {
            "pt1" => Some(Box::new(day2_pt1)),
            "pt2" => Some(Box::new(day2_pt2)),
            _ => Some(Box::new(day2_pt1)),
        },
        "day3" => match part {
            "pt1" => Some(Box::new(day3_pt1)),
            "pt2" => Some(Box::new(day3_pt2)),
            _ => Some(Box::new(day3_pt1)),
        },
        "day4" => match part {
            "pt1" => Some(Box::new(day4_pt1)),
            "pt2" => Some(Box::new(day4_pt2)),
            _ => Some(Box::new(day4_pt1)),
        },
        // "day5" => match part {
        //     "pt1" => Some(day5_pt1),
        //     "pt2" => Some(day5_pt2),
        //     _ => Some(day5_pt1),
        // },
        str @ _ => {
            let out = format!("Invalid day selected: {str}");
            console_log(&out);
            None
        }
    }
}

/// A parameterized incrementing button
#[component]
pub fn Aoc_input() -> impl IntoView {
    let main_input = create_node_ref::<Textarea>();
    let main_select = create_node_ref::<Select>();
    let (part_name, set_name) = create_signal("pt1".to_string());
    let (result, set_result) = create_signal("".to_string());
    let on_click = move |_| {
        let input = main_input
            .get()
            .expect("should be loaded before submitting");
        let day = main_select
            .get()
            .expect("should be loaded before submitting");
        let f = aoc_func(day.value().as_str(), part_name().as_str());

        let res = f.unwrap()(input.value()).to_string();
        set_result(res);
    };

    // Log changes to result in console
    create_effect(move |_| {
        let res = result();
        if res == "" {
            return;
        }
        let fmt_str = format!("UPDATE: {}", res);
        console_log(&fmt_str);
    });

    view! {
            <div style="width:75%;height:400px">
                <textarea node_ref=main_input type="text" id="main_input" style="width:100%;height:100%"/>
            </div>
            <div height="200px" class="buttons">
                <select node_ref=main_select id="day" name="day">
                  <option value="day1">"Day 1"</option>
                  <option value="day2">"Day 2"</option>
                  <option value="day3">"Day 3"</option>
                  <option value="day4">"Day 4"</option>
                </select>

                <form name="selected_part" id="selected_part"
                    value=part_name
                    style="text-align:left">
                    <input type="radio" id="pt1" name="part_select"
                        checked
                        on:input=move |_| {set_name("pt1".to_string());}
                        value="pt1"/>
                    <label for="pt1">"Part One"</label><br/>
                    <input type="radio" id="pt2" name="part_select"
                        on:input=move |_| {set_name("pt2".to_string());}
                        value="pt2"/>
                    <label for="pt2">"Part Two"</label><br/>
                </form>

                <button on:click=on_click>
                    "Process Input"
                </button>
                <h4>
                    Result: {move || result()}
                </h4>
            </div>
    }
}
