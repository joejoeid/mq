use anyhow::Result;
use mailparse::*;
use std::fs;
use std::path::PathBuf;

pub fn generate_preview(path: &PathBuf) -> Result<String> {
    let content = fs::read(path)?;
    let parsed = parse_mail(&content)?;
    
    // We want to extract plain text first. If not found, convert HTML to text.
    let body = extract_text(&parsed);
    
    // Replace non-breaking spaces and zero-width spaces with standard spaces
    // because some characters can break terminal rendering.
    // Also handling U+034F Combining Grapheme Joiner which can break alignment.
    let body = body
        .replace('\u{00A0}', " ") // Non-breaking space
        .replace('\u{200B}', "")  // Zero-width space
        .replace('\u{200C}', "")  // Zero-width non-joiner
        .replace('\u{200D}', "")  // Zero-width joiner
        .replace('\u{FEFF}', "")  // Byte order mark
        .replace('\u{034F}', "")  // Combining Grapheme Joiner
        .replace('\u{034F}', ""); // Combining Grapheme Joiner

    // To prevent bleeding over the right edge, let's aggressively truncate 
    // any single line to 120 chars, replacing with "..."
    let lines: Vec<String> = body.lines().take(500).map(|line| {
        if line.chars().count() > 120 {
            let mut truncated: String = line.chars().take(117).collect();
            truncated.push_str("...");
            truncated
        } else {
            line.to_string()
        }
    }).collect();
    
    let preview = lines.join("\n");
    
    Ok(preview)
}

fn extract_text(parsed: &ParsedMail) -> String {
    let mut plain_text = String::new();
    let mut html_text = String::new();

    extract_recursive(parsed, &mut plain_text, &mut html_text);

    if !plain_text.trim().is_empty() {
        plain_text
    } else if !html_text.trim().is_empty() {
        html2text::from_read(html_text.as_bytes(), 120).unwrap_or_else(|_| String::from("(Failed to parse HTML)"))
    } else {
        String::from("(No text body found)")
    }
}

fn extract_recursive(parsed: &ParsedMail, plain: &mut String, html: &mut String) {
    if parsed.ctype.mimetype == "text/plain" {
        if let Ok(body) = parsed.get_body() {
            plain.push_str(&body);
            plain.push('\n');
        }
    } else if parsed.ctype.mimetype == "text/html" {
        if let Ok(body) = parsed.get_body() {
            html.push_str(&body);
            html.push('\n');
        }
    }

    for sub in &parsed.subparts {
        extract_recursive(sub, plain, html);
    }
}

pub fn extract_html(path: &PathBuf) -> Result<Option<String>> {
    let content = fs::read(path)?;
    let parsed = parse_mail(&content)?;
    
    let mut plain_text = String::new();
    let mut html_text = String::new();
    extract_recursive(&parsed, &mut plain_text, &mut html_text);
    
    if !html_text.trim().is_empty() {
        Ok(Some(html_text))
    } else {
        Ok(None)
    }
}
