use colored::Colorize;

pub fn raise_exception(function_name : &str, string_message : &mut String, help : String, total_width: usize,  error_code : u16) {
    
    let horizontal_bar = &"_";
    let vertical_bar = &"|";

    let first_col_width = format!("fn <Matrix::<T>>.{}()", function_name).len() + 6;
    let second_col_width = total_width-2-first_col_width;
    let height = string_message.len()/second_col_width;
    string_message.extend(" ".repeat(second_col_width-string_message.len()%second_col_width).chars());

    print!("\n\n{}", format!("[NumRu raised exception (#{})] ",error_code).bold().red());
    print!("{}", " A linear algebra error occured with the Matrix object. See below for more details.\n".red());

    print!("{val}", val=horizontal_bar.repeat(total_width).red());
    print!("\n{val}{}{val}", " ".repeat(total_width-2), val = vertical_bar.red());

    for line_break in 0..=height {
        print!("\n{}", vertical_bar.red());
        if line_break == (height)/2 {
            print!(" {} {}", format!("fn <Matrix::<T>>.{}()", function_name).red().bold(), ">>> ".red());
        }
        else {
            
            print!("{val:>width$}", val="", width=first_col_width);
        }

        print!("{val:>width$}{}", vertical_bar.red(), val=&string_message[line_break*second_col_width..(line_break+1)*second_col_width], width=second_col_width);
            
    }
    print!("\n{val}{}{val}", horizontal_bar.repeat(total_width-2).red(), val="|".red());
    print!("\n{help_title} {help} \n\n", help_title=format!("Help (#{code})", code=error_code).green().bold(), help=help.italic().green());
    panic!("{}", format!("NumRu raised exception (#{}) ",error_code))
}