
use std::io::stdout;
use std::io::Error as IOError;

use std::marker::PhantomData;


use menuforge::crossterm; 
use menuforge::RawSelect;
use menuforge::KeysTrait;
use menuforge::RawConfigs;

use menuforge::KeyFunc;
use menuforge::SelErr;

use crossterm::event; 
use crossterm::queue; 
use crossterm::style::Print;
use crossterm::style::Color; 
use crossterm::style::Colors; 
use crossterm::style::SetColors; 
use crossterm::style::ResetColor;

use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;

const QUEUE_ERR:&'static str = "MOVE STRING";


pub type RetOk = ();
pub type RetErr = ();

pub type ActCtx<'a> = (usize, MenuStatus, &'a mut usize);
pub type PrintCtx = (usize, MenuStatus);

struct ExtraMenu<'a> {
    menu_status: MenuStatus,
    inner: RawSelect<String, ActCtx<'a>, PrintCtx, RetOk, RetErr>,
    keys: Keys,
    fields: Fields,
}

struct Keys {
}


#[derive(Default)]
struct Fields {
    status: MenuStatus,
    index: usize,
}


#[derive(Clone, Copy, Default)]
enum MenuStatus {
    #[default]
    Principal, 
    Confirmation,
}

fn main() {
    let mut menu:ExtraMenu = ExtraMenu::new(10);
    //let menu_ref:&mut ExtraMenu = &mut menu;
    let mut exit = 10i32;
    menu.init_prompt();
    
    let mut list = ["hello".to_string(), "test".to_string(), "mas".to_string()];
        let mut index = 0usize;
    loop {
        
        //menu.prompt(&list);
    }
    
    menu.end_prompt();
    println!("test");
}

impl<'a, 'b> KeysTrait<String, ActCtx<'a>, RetOk, RetErr> for Keys {
    fn get_key_action(&mut self, ctx:&'a mut ActCtx, event:&event::Event) -> Option<KeyFunc<String, ActCtx<'b>, RetOk, RetErr>> {
        /*
        match event {
            _ => {
                panic!();
            }
        }
        */
    }
}

#[allow(dead_code)]
fn nope(_:&[String], _:&mut ActCtx) -> Result<RetOk, SelErr<RetErr>> {
    Ok(())
}

impl<'a> ExtraMenu<'a> {
    
    const UP_ARROW:&'static str = " ^ ";
    const DOWN_ARROW:&'static str = " v ";
    
    fn new(table_size:u16) -> Self {
        let mut config_holder = RawConfigs::default();
        config_holder.table_size = table_size;
        let keys = Keys{};
        Self{
            menu_status: MenuStatus::Principal,
            keys,
            inner:RawSelect::<String, ActCtx<'a>, PrintCtx, (), ()>::new(config_holder),
            fields: Fields::default(),
        }
    }
    
    fn init_prompt(&mut self) {
        let _ = self.inner.init_prompt();
    }

    
    fn end_prompt(&mut self) {
        let _ = self.inner.end_prompt();
    }
    
    fn poll(&self) -> bool {
        self.inner.poll().unwrap()
    }
    
    fn prompt(&mut self, list:&[String]) {
    }
    
    fn print_line(&mut self, list:&[String]) {
        let ctx = (list.len(), self.fields.status);
        self.inner.print_line(list, ctx, Self::print_cbk).expect("TODO");
    }
    
    #[allow(dead_code)]
    fn print_cbk(line:u16, menu_size:u16, entries:&[String], print_ctx:&mut PrintCtx) -> Result<(), IOError> {
        let (index, status) = print_ctx.clone();
        match status {
            MenuStatus::Principal => {
                let half = menu_size/2;
                let pair_offset:usize = if menu_size%2==0 {1} else {0};
                let pair_complements:usize = if menu_size%2==0 {0} else {1};
                let current_index;
                let position_from_last = entries.len() - index ;
                
                if index < half.into() || entries.len() < (menu_size as usize) {
                    current_index = usize::try_from(line).unwrap();
                    if index == line.into() {
                        Self::selected_line().expect(QUEUE_ERR);
                    } else {
                        if line == menu_size - 1 && entries.len() > menu_size.into() {
                            Self::bottom_line().expect(QUEUE_ERR);
                        } else {
                            Self::empty_line().expect(QUEUE_ERR);
                        }
                    }
                    
                } else if position_from_last+pair_offset <= half.into() {
                    /* when cursor is at the bottom */
                    current_index = entries.len() - menu_size as usize + line as usize;
                    if line == menu_size - position_from_last as u16 {
                        Self::selected_line().expect(QUEUE_ERR);
                    } else {
                        if line == 0 && entries.len() > menu_size.into() {
                            Self::top_line().expect(QUEUE_ERR);
                        } else {
                            Self::empty_line().expect(QUEUE_ERR);
                        }
                    }
                    
                }  else {
                    current_index = index + line as usize - half as usize;
                    if line == half {
                        Self::selected_line().expect(QUEUE_ERR);
                    } else {
                        if line == 0 && index > half.into() {
                            Self::top_line().expect(QUEUE_ERR);
                        } else if line == menu_size - 1 && position_from_last - pair_complements > half.into() {
                            Self::bottom_line().expect(QUEUE_ERR);
                        } else {
                            Self::empty_line().expect(QUEUE_ERR);
                        }
                    }
                }
                if current_index < entries.len() {
                    queue!(
                        stdout(), 
                        Print(&entries[current_index]),
                        ResetColor,
                        Clear(ClearType::UntilNewLine)
                    ).expect(QUEUE_ERR);
                } else {
                    queue!(
                        stdout(), 
                        ResetColor,
                        Clear(ClearType::UntilNewLine)
                    ).expect(QUEUE_ERR);
                }
                Ok(())
            }
            MenuStatus::Confirmation => {
                todo!("new mode print");
            }
        }
    }
 
    fn selected_line() -> Result<(), IOError> {
        queue!(
            stdout(), 
            Print(" > "),
            SetColors(Colors::new(Color::Blue, Color::Black))
        )
    }
    
    fn empty_line() -> Result<(), IOError> {
        queue!(
            stdout(), 
            Print("   ")
        )
    }
    
    fn bottom_line() -> Result<(), IOError> {
        queue!(
            stdout(), 
            Print(Self::DOWN_ARROW)
        )
    }
    
    fn top_line() -> Result<(), IOError> {
        queue!(
            stdout(), 
            Print(Self::UP_ARROW)
        )
    }
    

}

