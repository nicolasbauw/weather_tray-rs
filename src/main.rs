#![windows_subsystem = "windows"]
mod image_creation;
mod request_maker;

//use std::path;

//#[cfg(target_os = "windows")]
fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    
    // We initialize an empty icon, so the program will not exit if request fails
    image_creation::cria_imagem("00");

    if let Ok(temp) = request_maker::get_temp() {
        image_creation::cria_imagem(&temp);
    }

    //Mudar para apenas o nome do arquivo no final
    app.set_icon_from_file(&"temp.ico".to_string())?;

    app.add_menu_item("Refresh", |window| {
        if let Ok(temp) = request_maker::get_temp() {
            image_creation::cria_imagem(&temp);
        }
        window.set_icon_from_file(&"temp.ico".to_string())?;
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}
