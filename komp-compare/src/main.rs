//use komp_compare::file_process::file_process::read_file;
use komp_compare::basic_methods::l_distance::l_distance;
use komp_compare::basic_methods::k_grams::k_grams;
use fltk::enums::CallbackTrigger;
use fltk::{app, prelude::*, window::Window, frame::Frame, button::Button, group::{Group, Wizard},
input:: MultilineInput};
use fltk::enums::Color;
use fltk::button;
use fltk::enums::FrameType;
use fltk::enums::Font;
use fltk::image::PngImage;
use fltk::enums::Event;
use std::fs::File;
use std::io::Read;
use fltk::dialog::NativeFileChooser;
use std::rc::Rc;
use std::cell::RefCell;
use fltk::dialog::alert;
use std::cell::Cell;

fn frame_format(frm : &mut Frame)
{
    frm.set_label_font(Font::HelveticaBold);
    frm.set_label_color(Color::White);
    frm.set_frame(FrameType::RFlatBox);
    frm.set_color(Color::from_rgb(58,191,142));
}
fn read_file_content(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn pick_and_read_file() -> Option<String> {
    let mut nfc = NativeFileChooser::new(fltk::dialog::NativeFileChooserType::BrowseFile);
    nfc.set_filter("*.txt");
    nfc.show();

    let path = nfc.filename();
    if path.display().to_string().is_empty() {
        return None;
    }

    let path_str = path.to_string_lossy().to_string();
    read_file_content(&path_str).ok()
}

fn setup_file_picker(frm: &mut Frame,stockage: Rc<RefCell<String>>) {
    frm.handle(move |zone, even| {
        match even {
            Event::Push => true,
            Event::Released =>
            {
                if let Some(contenu) = pick_and_read_file()
                {
                    *stockage.borrow_mut() = contenu.clone();
                    zone.set_label("Chargé !");
                }
                true
            },
            _ => false,
        }
    });
}

fn color_button(but : &mut button::Button)
{

    but.clear_visible_focus();
    but.set_frame(FrameType::RFlatBox);
    but.set_color(Color::from_rgb(119,89,255));
    but.set_selection_color(Color::from_rgb(119,89,255).darker());
    but.set_label_font(Font::HelveticaBold);
    but.set_label_color(Color::White);
}

fn backround()
{
    let mut fond_logo = Frame::new(0, 0, 800, 300, "");
    if let Ok(mut logo) = PngImage::load("logo_komp.png")
    {
       logo.scale(logo.width(),logo.height()/2,true,false); 
       fond_logo.set_image(Some(logo));
       fond_logo.deactivate();
    }
}
fn main()
{
    let k = Rc::new(Cell::new(4));
    let contenu_1 = Rc::new(RefCell::new(String::new()));
    let contenu_2 = Rc::new(RefCell::new(String::new()));

    let contenu_1c = Rc::new(RefCell::new(String::new()));
    let contenu_2c = Rc::new(RefCell::new(String::new()));
    //======================
    //======================

    let app = app::App::default();
    

    let mut wind = Window::new(100, 100, 800, 600, "Komp compare");

//page d'accueil
    let wizard = Wizard::default().size_of(&wind);
    
    let mut accueil = Group::default().size_of(&wizard);
    backround();
    accueil.set_color(Color::from_rgb(245, 247, 249));
    let mut but1 = Button::new(320,400,160,80,"Debuter");
    color_button(&mut but1);
    accueil.end();

//menu principal
    let mut menu_principal = Group::default().size_of(&wizard);
    menu_principal.set_color(Color::from_rgb(245, 247, 249));
    let mut frame_m = Frame::new(200,5,400,40,"Que voulez-vous comparer?");
    frame_m.set_label_size(20);
    frame_format(&mut frame_m);
    let mut but_m1 = Button::new(320,80,180,80,"Textes classiques");
    let mut but_m2 = Button::new(320,170,180,80,"Code source");
    let mut but_m3 = Button::new(320,260,180,80,"Comparaison rapide");
    let mut but_m4 = Button::new(320,440,180,80,"Retour");
    let mut but_m5 = Button::new(320,350,180,80,"Comparaison d'images");

    color_button(&mut but_m1);
    color_button(&mut but_m2);
    color_button(&mut but_m3);
    color_button(&mut but_m4);
    color_button(&mut but_m5);
    menu_principal.end();

//main menu -> Comparaison classique

    let mut comparaison_classique = Group::default().size_of(&wizard);
    comparaison_classique.set_color(Color::from_rgb(245, 247, 249));
    let mut but_cs1 = Button::new(610,500,160,80,"Retour");
    color_button(&mut but_cs1);
    let mut but_cs2 = Button::new(20,500,160,80,"Reinitialiser");
    color_button(&mut but_cs2);
    let mut but_compare_c = Button::new(320, 420, 160, 80, "Comparer les fichiers");
    color_button(&mut but_compare_c);

     let mut zone_1c =  Frame::new(150,300,200,100,"Cliquez pour choisir\nun fichier .txt");
    frame_format(&mut zone_1c);
    zone_1c.set_color(Color::from_rgb(215,215,215));
    setup_file_picker(&mut zone_1c, contenu_1c.clone());

    let mut zone_2c =  Frame::new(450,300,200,100,"Cliquez pour choisir\nun fichier .txt");
    frame_format(&mut zone_2c);
    zone_2c.set_color(Color::from_rgb(215,215,215));
    setup_file_picker(&mut zone_2c, contenu_2c.clone());

    let mut frm_cs = Frame::new(100,5,600,100,"Comparaison de textes");
    frame_format(&mut frm_cs);
    frm_cs.set_label_size(20);

    let mut frame_result_cs = Frame::new(100,80,600,70,"Similarité : 0%");
    frame_result_cs.set_label_size(20);
    frame_format(&mut frame_result_cs);


    comparaison_classique.end();

//main menu -> comparaison rapide
    
     let mut comparaison_rapide = Group::default().size_of(&wizard);
     comparaison_rapide.set_color(Color::from_rgb(245, 247, 249));
     let mut frame_result_cr = Frame::new(100,20,600,100,"Similarité : 0%\ndistance de Levenshtein : 0");
     frame_result_cr.set_label_size(20);
     frame_format(&mut frame_result_cr);
     let mut inp1 = MultilineInput::new(25,150,350, 250, "");
     let mut inp2 = MultilineInput::new(425,150,350, 250, "");
     inp1.set_trigger(CallbackTrigger::Changed);
     inp2.set_trigger(CallbackTrigger::Changed);

     let mut but_cr1 = Button::new(400-80,510,160,80,"Comparer");
     let mut but_cr2 = Button::new(800-120,520,120,60,"Retour");
     let mut but_cr3 = Button::new(400-80,420,160,80,"Reinitialiser");

     color_button(&mut but_cr1);
     color_button(&mut but_cr2);
     color_button(&mut but_cr3);
     comparaison_rapide.end();
//main menu -> comparaison code source
        
    let mut comparaison_code = Group::default().size_of(&wizard);
    comparaison_code.set_color(Color::from_rgb(245, 247, 249));
    let mut frm_ccs = Frame::new(100,5,600,100,"Comparaison de codes");
    frame_format(&mut frm_ccs);
    frm_ccs.set_label_size(20);
    let mut deco = Frame::new(95,295,610,110,"");
     frame_format(&mut deco);
     deco.set_color(Color::from_rgb(195,182,253));

      let mut but_ccs1 = Button::new(610,500,160,80,"Retour");
    color_button(&mut but_ccs1);

    let mut zone_1 =  Frame::new(150,300,200,100,"Cliquez pour choisir\nun fichier .txt");
    frame_format(&mut zone_1);
    zone_1.set_color(Color::from_rgb(215,215,215));
    setup_file_picker(&mut zone_1, contenu_1.clone());

    let mut zone_2 =  Frame::new(450,300,200,100,"Cliquez pour choisir\nun fichier .txt");
    frame_format(&mut zone_2);
    zone_2.set_color(Color::from_rgb(215,215,215));
    setup_file_picker(&mut zone_2, contenu_2.clone());

    let mut frame_result_ccs = Frame::new(100,80,600,70,"Similarité : 0%");
    frame_result_ccs.set_label_size(20);
    frame_format(&mut frame_result_ccs);

    let mut but_ccs3 = Button::new(20,500,160,80,"Reinitialiser");
    color_button(&mut but_ccs3);
    let mut but_compare = Button::new(320, 420, 160, 80, "Comparer les fichiers");
    color_button(&mut but_compare);
    comparaison_code.end();

//option de compa =====================

    let mut option_compa = Group::default().size_of(&wizard);
    option_compa.set_color(Color::from_rgb(245, 247, 249));
    let mut frm_op1 = Frame::new(100,50,600,50,"Quel type de texte souhaitez-vous comparer ?");
    frame_format(&mut frm_op1);

    let mut but_op1 = Button::new(610,500,160,80,"Retour");
    let mut but_op2 = Button::new(230,110,340,80,"Textes courts (Tweets, SMS, slogans)");
    let mut but_op3 = Button::new(230,200,340,80,"Textes standards (Articles, rédactions, ...)");
    let mut but_op4 = Button::new(230,290,340,80,"Textes longs et formels");
    color_button(&mut but_op1);
    color_button(&mut but_op2);
    color_button(&mut but_op3);
    color_button(&mut but_op4);
    
    option_compa.end();

    wind.end();
    wind.show();
    
  

    let cl1 = comparaison_classique.clone();
    let cl2 = comparaison_classique.clone();

    let mut valueccs = frame_result_ccs.clone();
    let mut valuecs = frame_result_cs.clone();

    let c1_handle = contenu_1.clone();
    let c2_handle = contenu_2.clone();

    let c1_handle2 = contenu_1c.clone();
    let c2_handle2 = contenu_2c.clone();

    let _result_frame = frm_ccs.clone();
    
    but_compare.set_callback(move |_| {
    let texte1 = c1_handle.borrow();
    let texte2 = c2_handle.borrow();

    if texte1.is_empty() || texte2.is_empty() {
        alert(200, 200, "Veuillez choisir deux fichiers !");
    } else {

        
        let score = k_grams(&texte1, &texte2, 3, false, true);
        frame_result_ccs.set_label(&format!("Similarité : {:.2}%", score));
    }
});

    but_ccs3.set_callback(move |_| {
    valueccs.set_label("Similarité : 0%");
    zone_1.set_label("Cliquez pour choisir\nun fichier .txt");
    zone_2.set_label("Cliquez pour choisir\nun fichier .txt");
    contenu_1.borrow_mut().clear();
    contenu_2.borrow_mut().clear(); 
});

 
    let k_op2 = k.clone();
    let mut wiz_clonel1 = wizard.clone();
    but_op2.set_callback(move |_| {
        k_op2.set(4);
        wiz_clonel1.set_current_widget(&comparaison_classique);});

    let k_op3 = k.clone();
    let mut wiz_clonel2 = wizard.clone();
    but_op3.set_callback(move |_| {
        k_op3.set(5);
        wiz_clonel2.set_current_widget(&cl1);});

    let k_op4 = k.clone();
    let mut wiz_clonel3 = wizard.clone();
    but_op4.set_callback(move |_| {
        k_op4.set(7);
        wiz_clonel3.set_current_widget(&cl2);});
    let k_compare = k.clone();    
    but_compare_c.set_callback(move |_|
        {
            let current_k = k_compare.get();
            println!("{:?}",current_k);
             let texte1 = c1_handle2.borrow();
             let texte2 = c2_handle2.borrow();
              if texte1.is_empty() || texte2.is_empty() {
        alert(200, 200, "Veuillez choisir deux fichiers !");
    }
              else { 
                    let score = k_grams(&texte1, &texte2, current_k, true, false);
                    frame_result_cs.set_label(&format!("Similarité : {:.2}%", score));
              }
        });

     but_cs2.set_callback(move |_| {
    valuecs.set_label("Similarité : 0%");
    zone_1c.set_label("Cliquez pour choisir\nun fichier .txt");
    zone_2c.set_label("Cliquez pour choisir\nun fichier .txt");
    contenu_1c.borrow_mut().clear();
    contenu_2c.borrow_mut().clear();
});

    
    let mut wiz_clone = wizard.clone();
    but1.set_callback(move |_| {wiz_clone.next()});

    let mut wiz_clone2 = wizard.clone();
    but_m4.set_callback(move |_| {wiz_clone2.prev()});


    let mut wiz_clone4 = wizard.clone();
    but_cs1.set_callback(move |_| {wiz_clone4.prev()});


    let mut wiz_clone3 = wizard.clone();
    but_m1.set_callback(move |_| {wiz_clone3.set_current_widget(&option_compa);});

     let mut wiz_clone5 = wizard.clone();
    but_m3.set_callback(move |_| {wiz_clone5.set_current_widget(&comparaison_rapide);});

    let value = menu_principal.clone();
    let value2 = menu_principal.clone();
    let mut wiz_clone6 = wizard.clone();
    but_cr2.set_callback(move |_| {wiz_clone6.set_current_widget(&menu_principal);});
//========
    let mut wiz_clonel = wizard.clone();
    but_op1.set_callback(move |_| {
        wiz_clonel.set_current_widget(&value2);});
   /*
    let mut wiz_clonel1 = wizard.clone();
    but_op2.set_callback(move |_| {
        k=4;
        wiz_clonel1.set_current_widget(&comparaison_classique);});

    let mut wiz_clonel2 = wizard.clone();
    but_op3.set_callback(move |_| {
        k=5;
        wiz_clonel2.set_current_widget(&cl1);});

    let mut wiz_clonel3 = wizard.clone();
    but_op4.set_callback(move |_| {
        k=7;
        wiz_clonel3.set_current_widget(&cl2);}); */
//=========
    let mut frame_c1 = frame_result_cr.clone();
    let inp1_c1 = inp1.clone();
    let inp2_c1 = inp2.clone();

    let mut frame_c2 = frame_result_cr.clone();
    let mut inp1_c2 = inp1.clone();
    let mut inp2_c2 = inp2.clone();

    but_cr1.set_callback(move |_| 
        {
            frame_c1.set_label(
                &format!("Similarité : {:.1}%\ndistance de Levenshtein : {}", 
                    k_grams(&inp1_c1.value(), &inp2_c1.value(), 3, false, false), 
                    l_distance(&inp1_c1.value(), &inp2_c1.value()))
          );
        });

    but_cr3.set_callback(move |_| {
        frame_c2.set_label("Similarité : 0%\ndistance de Levenshtein : 0");
        inp1_c2.set_value("");
        inp2_c2.set_value("");
    });
    
    let mut wiz_clone7 = wizard.clone();
    but_m2.set_callback(move|_| wiz_clone7.set_current_widget(&comparaison_code));
    

    let mut wiz_clone8 = wizard.clone();
    but_ccs1.set_callback(move|_| wiz_clone8.set_current_widget(&value));

    app.run().unwrap();

}

