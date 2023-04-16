use anyhow::{anyhow, Result};
use copypasta::{ClipboardContext, ClipboardProvider};

pub mod aes;
pub use {aes_gcm_siv,rand};

pub fn sha256(data: &[u8]) -> Result<String> {
    let val = sha256::digest(data);
    Ok(val)
}

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
    fn test_sha256() {
        let result = sha256("ni hao".as_bytes()).unwrap();
        println!("{:?}", result.as_bytes());
    }


    #[test]
    fn copy_clipboard() {
        let _ = copy_to_clipboard("hello world").unwrap();
        let context = text_from_clipboard().unwrap();
        println!("{}", context);
    }
}
