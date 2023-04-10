use anyhow::{anyhow, Result};
use copypasta::{ClipboardContext, ClipboardProvider};

pub mod aes;


pub fn copy_to_clipboard(msg: &str) -> Result<()> {
    let mut ctx = ClipboardContext::new().map_err(|e| anyhow!("{}",e))?;
    ctx.set_contents(msg.to_owned()).map_err(|e| anyhow!("{}",e))?;
    Ok(())
}

pub fn text_from_clipboard() -> Result<String> {
    let mut ctx = ClipboardContext::new().map_err(|e| anyhow!("{}",e))?;
    let content = ctx.get_contents().map_err(|e| anyhow!("{}",e))?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_clipboard() {
        let _ = copy_to_clipboard("hello world").unwrap();
        let context = text_from_clipboard().unwrap();
        println!("{}", context);
    }
}
