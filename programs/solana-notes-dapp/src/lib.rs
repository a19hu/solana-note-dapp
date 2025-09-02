use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("5yWh4mptkn4K45VxnxrSRmAQnRsb9WxwzKcrifMzTju3");

#[program]
pub mod notes_dapp {
    use super::*;
    pub fn create_note(ctx:Context<CreateNote>,title:String,content:String)-> Result<()> {
        let note = &mut ctx.accounts.note;
        let clock = Clock::get()?;
        
        note.author = ctx.accounts.author.key();
        note.title = title.clone();
        note.content = content.clone();
        note.create_at = clock.unix_timestamp;
        note.last_updated = clock.unix_timestamp;
        msg!(
            "Note Created Title {}, Author {}, Create_at {}",
            note.title,
            note.author,
            note.create_at
        );


        Ok(())
    }

    pub fn update_note(ctx:Context<UpdateNote>,content:String)-> Result<()> {
        let note = &mut ctx.accounts.note;
        let clock = Clock::get()?;

        note.content = content.clone();
        note.last_updated = clock.unix_timestamp;

        msg!(
            "Note {} update", note.title
        );


        Ok(())
    }

    pub fn delete_note(ctx:Context<DeleteNote>)-> Result<()> {
        let note = &mut ctx.accounts.note;
        msg!(
            "Note title {} , delete", note.title
        );


        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct CreateNote<'info>{
    #[account(
        init,
        space = 8 + Note::INIT_SPACE,
        payer = author,
        seeds= [b"note",author.key().as_ref(), title.as_ref()],
        bump
    )]
    pub note: Account<'info,Note>,
    #[account(mut)]
    pub author : Signer<'info>,

    pub system_program : Program<'info,System>

}


#[derive(Accounts)]
pub struct UpdateNote<'info>{
    #[account(
        mut,
        seeds = [b"note",author.key().as_ref(), note.title.as_ref()],
        bump
    )]
    pub note :Account<'info,Note>,
    pub author : Signer<'info>
}

#[derive(Accounts)]
pub struct DeleteNote<'info>{
     #[account(
        mut,
        seeds = [b"note",author.key().as_ref(), note.title.as_ref()],
        bump,
        close =  author
    )]
    pub note :Account<'info,Note>,
    #[account(mut)]
    pub author : Signer<'info>
}
#[account]
#[derive(InitSpace)]
pub struct Note {
    pub author : Pubkey,
    #[max_len(100)]
    pub title : String,
    #[max_len(1000)]
    pub content : String,
    pub create_at : i64,
    pub last_updated : i64
}

#[error_code]
pub enum NoteError {
    #[msg("Title cannot be longer than 100 chars")]
    TitleToolong,
    #[msg("content cannot be longer than 1000 chars")]
    ContentToolong    
}