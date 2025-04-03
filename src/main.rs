use std::io::stdout;
use std::io::Error as IOError;
use std::thread::sleep;
use std::time;

use menuforge::crossterm; 
use menuforge::RawSelect;
use menuforge::KeysTrait;
use menuforge::RawConfigs;
use menuforge::RawSelResult;

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
const GRANTED_ERR:&'static str = "MOVE STRING";


pub type ListType = String;
pub type RetOk = RetOkEnum;
pub type RetErr = ();
pub type ActCtx = (MenuStatus, *mut usize);
pub type PrintCtx = (MenuStatus, usize);


pub enum RetOkEnum {
    Nope,
    Update,
    Exit,
    PrimaryOk(PrimaryOk),
    ConfirmationOk(ConfirmationOk),
}

pub enum PrimaryOk {
    Select,
}

pub enum ConfirmationOk {
    Back,
    Delete(usize),
}

pub struct ExtraMenu {
    inner: RawSelect<ListType, ActCtx, PrintCtx, RetOk, RetErr>,
    keys: Keys,
    fields: Fields,
}

struct Keys {}


#[derive(Default)]
pub struct Fields {
    status: MenuStatus,
    index: usize,
}


#[derive(Clone, Copy, Default, Debug)]
pub enum MenuStatus {
    #[default]
    Principal, 
    Confirmation,
}

fn list() -> Vec<ListType> {
    let mut holder = Vec::default();
    for elem in 0..16 {
        holder.push(format!("key{}", elem));
    }
    holder
}

fn main() {
    let mut menu:ExtraMenu = ExtraMenu::new(10);
    menu.init_prompt().expect(GRANTED_ERR);
    
    
    let mut list = list();
    menu.print_buffer(&list);
    
    loop {
        if menu.poll() {
            match menu.prompt(&list) {
                RawSelResult::Ok(holder) => {
                    match holder {
                        RetOk::Exit => {
                            break;
                        }
                        RetOk::Update => {
                            menu.print_buffer(&list);
                        }
                        RetOk::Nope => {
                            
                        }
                        RetOk::PrimaryOk(prim_ok) => {
                            match prim_ok {
                                PrimaryOk::Select => {
                                    menu.fields.status = MenuStatus::Confirmation;
                                    menu.print_buffer(&list);
                                }
                            }
                        }
                        RetOk::ConfirmationOk(confirmation_ok) => {
                            match confirmation_ok {
                                ConfirmationOk::Back => {
                                    menu.fields.status = MenuStatus::Principal;
                                    menu.print_buffer(&list);
                                }
                                ConfirmationOk::Delete(index) => {
                                    menu.fields.status = MenuStatus::Principal;
                                    list.remove(index);
                                    menu.print_buffer(&list);
                                }
                            }
                        }
                    }
                }
                _ => {
                    
                }
            }
        }
        sleep(time::Duration::from_millis(10));
    }
    
    menu.end_prompt().expect(GRANTED_ERR);
}

impl KeysTrait<ListType, ActCtx, RetOk, RetErr> for Keys {
    fn get_key_action(&mut self, ctx:&mut ActCtx, event:&event::Event) -> Option<KeyFunc<ListType, ActCtx, RetOk, RetErr>> {
        let act = ctx.0;
        match act {
            MenuStatus::Principal => {
                match event {
                    event::Event::Key(event::KeyEvent{
                        code:event::KeyCode::Enter,
                        ..
                    }) => {
                        Some(select)
                    }
                    event::Event::Key(event::KeyEvent{
                        code:event::KeyCode::Up,
                        ..
                    }) => {
                        Some(move_cursor_up)
                    }
                    event::Event::Key(event::KeyEvent{
                        code:event::KeyCode::Down,
                        ..
                    }) => {
                        Some(move_cursor_down)
                    }
                    event::Event::Key(event::KeyEvent{
                        code:event::KeyCode::Char('k'),
                        ..
                    }) => {
                        Some(move_cursor_up)
                    }
                    event::Event::Key(event::KeyEvent{
                        code:event::KeyCode::Char('j'),
                        ..
                    }) => {
                        Some(move_cursor_down)
                    }
                    event::Event::Key(event::KeyEvent{
                        code:event::KeyCode::Char('q'),
                        ..
                    }) => {
                        Some(exit)
                    }
                    _ => {
                        Some(nope)
                    }
                }
            }
            MenuStatus::Confirmation => {
                match event {
                    event::Event::Key(event::KeyEvent{
                        code:event::KeyCode::Char('d'),
                        ..
                    }) => {
                        Some(delete)
                    }
                    _ => {
                        Some(back)
                    }
                }
            }
        }
    }
}

fn nope(_:&[ListType], _:&mut ActCtx) -> Result<RetOk, SelErr<RetErr>> {
    Ok(RetOk::Nope)
}

fn exit(_:&[ListType], _:&mut ActCtx) -> Result<RetOk, SelErr<RetErr>> {
    Ok(RetOk::Exit)
}

fn move_cursor_up(_:&[ListType], act_ctx:&mut ActCtx) -> Result<RetOk, SelErr<RetErr>> {
    let index = unsafe{&mut *act_ctx.1};
    if *index > 0 {
        *index -= 1;
    }
    Ok(RetOk::Update)
}

fn move_cursor_down(list:&[ListType], act_ctx:&mut ActCtx) -> Result<RetOk, SelErr<RetErr>> {
    let index = unsafe{&mut *act_ctx.1};
    let len = list.len();
    if *index+1 < len {
        *index += 1;
    }
    Ok(RetOk::Update)
}

fn select(list:&[ListType], _:&mut ActCtx) -> Result<RetOk, SelErr<RetErr>> {
    if list.len() == 0 {
        Err(SelErr::UserErr(()))
    } else {
        Ok(RetOk::PrimaryOk(PrimaryOk::Select))
    }
}

fn back(_:&[ListType], _:&mut ActCtx) -> Result<RetOk, SelErr<RetErr>> {
    Ok(RetOk::ConfirmationOk(ConfirmationOk::Back))
}

fn delete(list:&[ListType], act_ctx:&mut ActCtx) -> Result<RetOk, SelErr<RetErr>> {
    let index = unsafe{&mut *act_ctx.1};
    let holder = index.clone();
    let len = list.len();
    
    if holder != 0 && holder+1 == len {
        *index -= 1;
    }
    Ok(RetOk::ConfirmationOk(ConfirmationOk::Delete(holder)))
}

impl ExtraMenu {
    
    const UP_ARROW:&'static str = " ^ ";
    const DOWN_ARROW:&'static str = " v ";
    
    fn new(table_size:u16) -> Self {
        let mut config_holder = RawConfigs::default();
        config_holder.table_size = table_size;
        let keys = Keys{};
        Self{
            keys,
            inner:RawSelect::<ListType, ActCtx, PrintCtx, RetOk, RetErr>::new(config_holder),
            fields: Fields::default(),
        }
    }
    
    fn init_prompt(&mut self) -> Result<(), IOError> {
        self.inner.init_prompt()
    }

    
    fn end_prompt(&mut self) -> Result<(), IOError> {
        self.inner.end_prompt()
    }
    
    fn poll(&self) -> bool {
        self.inner.poll().unwrap()
    }
    
    fn prompt(&mut self, list:&[ListType]) -> RawSelResult<RetOk, SelErr<RetErr>> {
        let mut ctx = (self.fields.status, &mut self.fields.index as *mut usize);
        self.inner.raw_prompt(&mut self.keys, list, &mut ctx)
    }
    
    fn print_buffer(&mut self, list:&[ListType]) {
        let ctx = (self.fields.status, self.fields.index);
        self.inner.print_buffer(list, ctx, Self::print_cbk).expect(GRANTED_ERR);
    }
    
    #[allow(dead_code)]
    fn print_cbk(line:u16, real_menu_size:u16, entries:&[ListType], print_ctx:&mut PrintCtx) -> Result<(), IOError> {
        let menu_size = real_menu_size-1;
        let (status, index) = print_ctx.clone();
        match status {
            MenuStatus::Principal => {
                Self::print_principal(line, menu_size, entries, index)
            }
            MenuStatus::Confirmation => {
                Self::print_confirmation(line, menu_size, entries, index)
            }
        }
    }
    
    fn print_principal(line:u16, menu_size:u16, entries:&[ListType], index:usize) -> Result<(), IOError> {
        if line >= menu_size {
            return Ok(());
        }
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
    
    fn print_confirmation(line:u16, _menu_size:u16, entries:&[ListType], index:usize) -> Result<(), IOError> {
        match line {
            0 => {
                queue!(
                    stdout(), 
                    Print(&entries[index]),
                    ResetColor,
                    Clear(ClearType::UntilNewLine)
                ).expect(QUEUE_ERR);
            }
            1 => {
                queue!(
                    stdout(), 
                    Print(&"(D)         delete"),
                    ResetColor,
                    Clear(ClearType::UntilNewLine)
                ).expect(QUEUE_ERR);
            }
            2 => {
                queue!(
                    stdout(), 
                    Print(&"(M)         modify"),
                    ResetColor,
                    Clear(ClearType::UntilNewLine)
                ).expect(QUEUE_ERR);
            }
            _ => {
                queue!(
                    stdout(), 
                    Clear(ClearType::UntilNewLine)
                ).expect(QUEUE_ERR);
            }
        }
        Ok(())
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

