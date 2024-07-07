//ui gw sampah amat yak

use gtk::{prelude::*, Entry};
use gtk::{
    Button, Label, TextView, Window, WindowType, Grid, Box, FileChooserDialog, FileChooserAction, ResponseType,
};
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::rc::Rc;

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new window wrapped in Rc
    let window = Rc::new(Window::new(WindowType::Toplevel));
    window.set_title("Array GUI");
    window.set_default_size(600, 600);

    // Create a grid to organize widgets
    let grid = Grid::new();
    grid.set_row_spacing(10);
    grid.set_column_spacing(10);
    window.add(&grid);

    // Create labels and TextView widgets
    let label = Label::new(Some(
       
        "Use a blank line to separate layers. Each row should be on a new line. Empty lines are used to separate layers.
        
        \n\
        Example:\n\
        1,2,3,4\n\
        5,6,7,8\n\
        9,10,11,12\n\
        \n\
        13,14,15,16\n\
        17,18,19,20\n\
        21,22,23,24"
    ));

    // Use TextView for multi-line input
    let text_view = TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_size_request(100, 100); // Set size for TextView

    let result_label = Rc::new(RefCell::new(Label::new(Some("Array content:"))));
    let max_label = Rc::new(RefCell::new(Label::new(Some("Max value:"))));
    let min_label = Rc::new(RefCell::new(Label::new(Some("Min value:"))));
    let mean_label = Rc::new(RefCell::new(Label::new(Some("Mean:"))));
    let median_label = Rc::new(RefCell::new(Label::new(Some("Median:"))));
    let stddev_label = Rc::new(RefCell::new(Label::new(Some("Standard Deviation:"))));
    let search_entry = Entry::new();
    search_entry.set_size_request(150, -1); // Set size for Entry
    let search_label = Label::new(Some("Search value:"));
    let filter_entry = Entry::new();
    filter_entry.set_size_request(150, -1); // Set size for Entry
    let filter_label = Label::new(Some("Filter threshold:"));

    // Create boxes to group buttons
    let button_box = Box::new(gtk::Orientation::Vertical, 10);
    button_box.set_size_request(-1, 300); // Set size for button box

    let process_button = Button::with_label("Process Array");
    let sort_button = Button::with_label("Sort Array");
    let stats_button = Button::with_label("Calculate Statistics");
    let remove_dups_button = Button::with_label("Remove Duplicates");
    let search_button = Button::with_label("Search Array");
    let filter_button = Button::with_label("Filter Array");
    let export_button = Button::with_label("Export Array");
    let import_button = Button::with_label("Import Array");

    // Set size for buttons
    for button in &[&process_button, &sort_button, &stats_button, &remove_dups_button, &search_button, &filter_button, &export_button, &import_button] {
        button.set_size_request(200, 40);
    }

    button_box.pack_start(&process_button, false, false, 0);
    button_box.pack_start(&sort_button, false, false, 0);
    button_box.pack_start(&stats_button, false, false, 0);
    button_box.pack_start(&remove_dups_button, false, false, 0);
    button_box.pack_start(&search_button, false, false, 0);
    button_box.pack_start(&filter_button, false, false, 0);
    button_box.pack_start(&export_button, false, false, 0);
    button_box.pack_start(&import_button, false, false, 0);

    // Add widgets to the grid
    grid.attach(&label, 0, 0, 2, 1);
    grid.attach(&text_view, 0, 1, 2, 1);
    grid.attach(&*result_label.borrow(), 0, 2, 2, 1);
    grid.attach(&*max_label.borrow(), 0, 3, 1, 1);
    grid.attach(&*min_label.borrow(), 1, 3, 1, 1);
    grid.attach(&*mean_label.borrow(), 0, 4, 1, 1);
    grid.attach(&*median_label.borrow(), 1, 4, 1, 1);
    grid.attach(&*stddev_label.borrow(), 0, 5, 2, 1);
    grid.attach(&search_label, 0, 6, 1, 1);
    grid.attach(&search_entry, 1, 6, 1, 1);
    grid.attach(&filter_label, 0, 7, 1, 1);
    grid.attach(&filter_entry, 1, 7, 1, 1);
    grid.attach(&button_box, 0, 8, 2, 1);

    // Shared state for the arrays
    let array_data = Rc::new(RefCell::new(Vec::new()));

    // Helper function to parse 2D and 3D arrays
    fn parse_array(input: &str) -> Vec<Vec<Vec<i32>>> {
        let mut result = Vec::new();
        let layers: Vec<&str> = input.split("\n\n").collect();
        for layer in layers {
            let rows: Vec<&str> = layer.split('\n').collect();
            let mut layer_vec = Vec::new();
            for row in rows {
                if row.trim().is_empty() {
                    continue;
                }
                let row_vec: Vec<i32> = row
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                if !row_vec.is_empty() {
                    layer_vec.push(row_vec);
                }
            }
            if !layer_vec.is_empty() {
                result.push(layer_vec);
            }
        }
        result
    }

    // Process Button Handler
    process_button.connect_clicked({
        let text_view = text_view.clone();
        let result_label = result_label.clone();
        let array_data = array_data.clone();
        move |_| {
            let buffer = text_view.get_buffer().expect("Failed to get buffer");
            let input = buffer.get_text(&buffer.get_start_iter(), &buffer.get_end_iter(), true).expect("Failed to get text");
            let array = parse_array(&input);
            *array_data.borrow_mut() = array.clone();

            let array_str = format!("{:?}", array);
            result_label.borrow_mut().set_text(&format!("Array content: {}", array_str));
        }
    });

    // Sort Button Handler
    sort_button.connect_clicked({
        let array_data = array_data.clone();
        let result_label = result_label.clone();
        move |_| {
            let mut array = array_data.borrow_mut();
            for layer in &mut *array {
                for row in &mut *layer {
                    row.sort();
                }
            }
            let array_str = format!("{:?}", *array);
            result_label.borrow_mut().set_text(&format!("Sorted Array: {}", array_str));
        }
    });

    // Statistics Button Handler
    stats_button.connect_clicked({
        let array_data = array_data.clone();
        let max_label = max_label.clone();
        let min_label = min_label.clone();
        let mean_label = mean_label.clone();
        let median_label = median_label.clone();
        let stddev_label = stddev_label.clone();
        move |_| {
            let array = array_data.borrow();
            let mut flat_array: Vec<i32> = array
                .iter()
                .flat_map(|layer| layer.iter().flat_map(|row| row.iter()))
                .copied()
                .collect();
            
            if flat_array.is_empty() {
                max_label.borrow_mut().set_text("Max value: N/A");
                min_label.borrow_mut().set_text("Min value: N/A");
                mean_label.borrow_mut().set_text("Mean: N/A");
                median_label.borrow_mut().set_text("Median: N/A");
                stddev_label.borrow_mut().set_text("Standard Deviation: N/A");
                return;
            }
    
            // Compute max and min
            let max = *flat_array.iter().max().unwrap_or(&0);
            let min = *flat_array.iter().min().unwrap_or(&0);
    
            // Compute mean
            let sum: i32 = flat_array.iter().sum();
            let mean = sum as f64 / flat_array.len() as f64;
    
            // Sort for median and standard deviation
            flat_array.sort();
            let median = if flat_array.len() % 2 == 0 {
                let mid = flat_array.len() / 2;
                (flat_array[mid - 1] + flat_array[mid]) as f64 / 2.0
            } else {
                flat_array[flat_array.len() / 2] as f64
            };
    
            let variance: f64 = flat_array
                .iter()
                .map(|&x| (x as f64 - mean).powi(2))
                .sum::<f64>() / flat_array.len() as f64;
            let stddev = variance.sqrt();
    
            max_label.borrow_mut().set_text(&format!("Max value: {}", max));
            min_label.borrow_mut().set_text(&format!("Min value: {}", min));
            mean_label.borrow_mut().set_text(&format!("Mean: {:.2}", mean));
            median_label.borrow_mut().set_text(&format!("Median: {:.2}", median));
            stddev_label.borrow_mut().set_text(&format!("Standard Deviation: {:.2}", stddev));
        }
    });

    // Remove Duplicates Button Handler
    remove_dups_button.connect_clicked({
        let array_data = array_data.clone();
        let result_label = result_label.clone();
        move |_| {
            let mut array = array_data.borrow_mut();
            for layer in &mut *array {
                for row in &mut *layer {
                    row.sort();
                    row.dedup();
                }
            }
            let array_str = format!("{:?}", *array);
            result_label.borrow_mut().set_text(&format!("Array with Duplicates Removed: {}", array_str));
        }
    });

    // Search Button Handler
    search_button.connect_clicked({
        let array_data = array_data.clone();
        let search_entry = search_entry.clone();
        let result_label = result_label.clone();
        move |_| {
            let search_value: i32 = match search_entry.get_text().parse() {
                Ok(value) => value,
                Err(_) => {
                    result_label.borrow_mut().set_text("Invalid search value.");
                    return;
                }
            };
            let array = array_data.borrow();
            let mut found = false;
            for layer in &*array {
                for row in &*layer {
                    if row.contains(&search_value) {
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if found {
                result_label.borrow_mut().set_text(&format!("Value {} found in the array.", search_value));
            } else {
                result_label.borrow_mut().set_text(&format!("Value {} not found in the array.", search_value));
            }
        }
    });

    // Filter Button Handler
    filter_button.connect_clicked({
        let array_data = array_data.clone();
        let filter_entry = filter_entry.clone();
        let result_label = result_label.clone();
        move |_| {
            let threshold: i32 = match filter_entry.get_text().parse() {
                Ok(value) => value,
                Err(_) => {
                    result_label.borrow_mut().set_text("Invalid filter threshold.");
                    return;
                }
            };
            let array = array_data.borrow();
            let filtered_array: Vec<Vec<Vec<i32>>> = array
                .iter()
                .map(|layer| {
                    layer
                        .iter()
                        .map(|row| row.iter().filter(|&&val| val >= threshold).copied().collect())
                        .filter(|row: &Vec<i32>| !row.is_empty())
                        .collect()
                })
                .filter(|layer: &Vec<Vec<i32>>| !layer.is_empty())
                .collect();
            let array_str = format!("{:?}", filtered_array);
            result_label.borrow_mut().set_text(&format!("Filtered Array: {}", array_str));
        }
    });

    // Export Button Handler
    export_button.connect_clicked({
        let array_data = array_data.clone();
        let window = window.clone();
        move |_| {
            let file_dialog = FileChooserDialog::with_buttons(
                Some("Save File"),
                Some(&*window),
                FileChooserAction::Save,
                &[("Save", ResponseType::Accept), ("Cancel", ResponseType::Cancel)],
            );
            if file_dialog.run() == ResponseType::Accept {
                let filename = file_dialog.get_filename().expect("Failed to get filename");
                let mut file = BufWriter::new(File::create(filename).expect("Failed to create file"));
                let array = array_data.borrow();
                let array_str = format!("{:?}", *array);
                writeln!(file, "{}", array_str).expect("Failed to write to file");
            }
            file_dialog.close();
        }
    });

    // Import Button Handler
    import_button.connect_clicked({
        let array_data = array_data.clone();
        let result_label = result_label.clone();
        let window = window.clone();
        move |_| {
            let file_dialog = FileChooserDialog::with_buttons(
                Some("Open File"),
                Some(&*window),
                FileChooserAction::Open,
                &[("Open", ResponseType::Accept), ("Cancel", ResponseType::Cancel)],
            );
            if file_dialog.run() == ResponseType::Accept {
                let filename = file_dialog.get_filename().expect("Failed to get filename");
                match File::open(&filename) {
                    Ok(file) => {
                        let reader = BufReader::new(file);
                        let mut content = String::new();
                        for line in reader.lines() {
                            content.push_str(&line.expect("Failed to read line"));
                            content.push('\n');
                        }
                        let array = parse_array(&content);
                        if array.is_empty() {
                            result_label.borrow_mut().set_text("Imported file is empty or invalid.");
                        } else {
                            *array_data.borrow_mut() = array.clone();
                            let array_str = format!("{:?}", array);
                            result_label.borrow_mut().set_text(&format!("Imported Array: {}", array_str));
                        }
                    }
                    Err(_) => result_label.borrow_mut().set_text("Failed to open file. Check file permissions or file format."),
                }
            }
            file_dialog.close();
        }
    });

    // Show all widgets
    window.show_all();

    // Run the GTK main loop
    gtk::main();
}