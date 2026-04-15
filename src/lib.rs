#![no_std]
#![doc(html_favicon_url = /*inline_data_url("./src/favicon.ico", "image/x-icon")*/"data:image/x-icon;base64,AAABAAMAEBAAAAEAIABoBAAANgAAACAgAAABACAAKBEAAJ4EAAAwMAAAAQAgAGgmAADGFQAAKAAAABAAAAAgAAAAAQAgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAP+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP8AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAKAAAACAAAABAAAAAAQAgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAP+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP8AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAKAAAADAAAABgAAAAAQAgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAP+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A//+/AP//vwD//78A/wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==")]
#![doc(html_logo_url = /*inline_data_url("./src/logo.png", "image/png")*/"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAQAAAAEACAIAAADTED8xAAACxUlEQVR4nOzTMRHAMAzAQF+v/CkFWgLDg/4RaNE/5w5UfdsBsMkApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUgzAGkGIM0ApBmANAOQZgDSDECaAUh7AQAA//85zAPBPp1gRgAAAABJRU5ErkJggg==")]
//!
//! The documentation for this Rust package contains quotation excerpts from
//! [RFC 9651: Structured Field Values for
//! HTTP](https://www.rfc-editor.org/rfc/rfc9651).
//!
//! > # Copyright Notice
//! >
//! > Copyright (c) 2024 IETF Trust and the persons identified as the document
//! > authors. All rights reserved.
//! >
//! > This document is subject to BCP 78 and the IETF Trust's Legal Provisions
//! > Relating to IETF Documents (https://trustee.ietf.org/license-info) in
//! > effect on the date of publication of this document. Please review these
//! > documents carefully, as they describe your rights and restrictions with
//! > respect to this document. Code Components extracted from this document
//! > must include Revised BSD License text as described in Section 4.e of the
//! > Trust Legal Provisions and are provided without warranty as described in
//! > the Revised BSD License.
//!
//! > # 1.1. Intentionally Strict Processing
//! >
//! > This specification intentionally defines strict parsing and serialization
//! > behaviors using step-by-step algorithms; the only error handling defined
//! > is to fail the entire operation altogether.
//! >
//! > It is designed to encourage faithful implementation and good
//! > interoperability. Therefore, an implementation that tried to be helpful by
//! > being more tolerant of input would make interoperability worse, since that
//! > would create pressure on other implementations to implement similar (but
//! > likely subtly different) workarounds.
//! >
//! > In other words, strict processing is an intentional feature of this
//! > specification; it allows non-conformant input to be discovered and
//! > corrected by the producer early and avoids both interoperability and
//! > security issues that might otherwise result.
//! >
//! > Note that as a result of this strictness, if a field is appended to by
//! > multiple parties (e.g., intermediaries or different components in the
//! > sender), an error in one party's value is likely to cause the entire field
//! > value to fail parsing.
//!
//! &mdash; [1.1. Intentionally Strict Processing | RFC
//! 9651](https://www.rfc-editor.org/rfc/rfc9651#name-intentionally-strict-proces)
//!
//! > # 2.2. Error Handling
//! >
//! > When parsing fails, the entire field is ignored (see Section 4.2). Field
//! > definitions cannot override this because doing so would preclude handling
//! > by generic software; they can only add additional constraints (for
//! > example, on the numeric range of Integers and Decimals, the format of
//! > Strings and Tokens, the types allowed in a Dictionary's values, or the
//! > number of Items in a List).
//! >
//! > When field-specific constraints are violated, the entire field is also
//! > ignored, unless the field definition defines other handling requirements.
//! > For example, if a header field is defined as an Item and required to be an
//! > Integer, but a String is received, it should be ignored unless that
//! > field's definition explicitly specifies otherwise.
//!
//! &mdash; [2.2. Error Handling | RFC
//! 9651](https://www.rfc-editor.org/rfc/rfc9651#name-error-handling)
//!
//! > # 6. Security Considerations
//! >
//! > The size of most types defined by Structured Fields is not limited; as a
//! > result, extremely large fields could be an attack vector (e.g., for
//! > resource consumption). Most HTTP implementations limit the sizes of
//! > individual fields as well as the overall header or trailer section size to
//! > mitigate such attacks.
//! >
//! > It is possible for parties with the ability to inject new HTTP fields to
//! > change the meaning of a Structured Field. In some circumstances, this will
//! > cause parsing to fail, but it is not possible to reliably fail in all such
//! > circumstances.
//! >
//! > The Display String type can convey any possible Unicode code point without
//! > sanitization; for example, they might contain unassigned code points,
//! > control points (including NUL), or noncharacters. Therefore, applications
//! > consuming Display Strings need to consider strategies such as filtering or
//! > escaping untrusted content before displaying it. See
//! > [[PRECIS](https://www.rfc-editor.org/rfc/rfc9651#RFC8264)] and
//! > [[UNICODE-SECURITY](https://www.rfc-editor.org/rfc/rfc9651#UNICODE-SECURITY)].
//!
//! &mdash; [6. Security Considerations | RFC
//! 9651](https://www.rfc-editor.org/rfc/rfc9651#name-security-considerations)

// mod abnf;

mod ascii_string;

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::{string::String, vec::Vec};
use core::{
    fmt::{self, Display},
    mem::transmute,
    ops::Deref,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct InsertionOrderedMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

/// > Parameters are an ordered map of key-value pairs that are associated with
/// > an Item ([Section 3.3](https://www.rfc-editor.org/rfc/rfc9651#item))
/// > or Inner List ([Section
/// > 3.1.1](https://www.rfc-editor.org/rfc/rfc9651#inner-list)). The keys
/// > are unique within the scope of the Parameters they occur within, and the
/// > values are bare items (i.e., they themselves cannot be parameterized;
/// > see [Section 3.3](https://www.rfc-editor.org/rfc/rfc9651#item)).
/// >
/// > Implementations MUST provide access to Parameters both by index and by
/// > key. Specifications MAY use either means of accessing them.
/// >
/// > Note that parameters are ordered, and parameter keys cannot contain
/// > uppercase letters.
/// >
/// > When serialized in a textual HTTP field, a Parameter is separated from its
/// > Item or Inner List and other Parameters by a semicolon. For example:
/// >
/// > ```http
/// > Example-List: abc;a=1;b=2; cde_456, (ghi;jk=4 l);q="9";r=w
/// > ```
/// >
/// > Parameters whose value is Boolean (see [Section
/// > 3.3.6](https://www.rfc-editor.org/rfc/rfc9651#boolean))
/// > true MUST omit that value when serialized. For example, the "a" parameter
/// > here is true, while the "b" parameter is false:
/// >
/// > ```http
/// > Example-Integer: 1; a; b=?0
/// > ```
/// >
/// > Note that this requirement is only on serialization; parsers are still
/// > required to correctly handle the true value when it appears in a
/// > parameter.
/// >
/// > Parsers MUST support at least 256 parameters on an Item or Inner List, and
/// > support parameter keys with at least 64 characters. Field specifications
/// > can constrain the order of individual parameters, as well as their values'
/// > types as required.
///
/// &mdash; [3.1.2. Parameters | RFC 9651](https://www.rfc-editor.org/rfc/rfc9651#name-parameters)
pub struct Parameters {
    // key_value_pairs: InsertionOrderedMap<ParameterKey, ParameterValue>,
}

/// > Parsers MUST support at least 256 parameters on an Item or Inner List, and
/// > support parameter keys with at least 64 characters. Field specifications
/// > can constrain the order of individual parameters, as well as their values'
/// > types as required.
///
/// &mdash; [3.1.2. Parameters | RFC 9651](https://www.rfc-editor.org/rfc/rfc9651#name-parameters)
pub const MAX_PARAMETERS_LEN: usize = 256;

/// > Parsers MUST support at least 256 parameters on an Item or Inner List, and
/// > support parameter keys with at least 64 characters. Field specifications
/// > can constrain the order of individual parameters, as well as their values'
/// > types as required.
///
/// &mdash; [3.1.2. Parameters | RFC 9651](https://www.rfc-editor.org/rfc/rfc9651#name-parameters)
pub const MAX_PARAMETER_KEY_LEN: usize = 64;

// pub struct Parameterized<T: ItemOrInnerList> {}

/// TODO
///
/// > Lists are arrays of zero or more members, each of which can be an Item
/// > ([Section 3.3](https://www.rfc-editor.org/rfc/rfc9651#item)) or an
/// > Inner List ([Section
/// > 3.1.1](https://www.rfc-editor.org/rfc/rfc9651#inner-list)), both of
/// > which can be Parameterized ([Section
/// > 3.1.2](https://www.rfc-editor.org/rfc/rfc9651#param)).
/// >
/// > An empty List is denoted by not serializing the field at all. This implies
/// > that fields defined as Lists have a default empty value.
/// >
/// > When serialized as a textual HTTP field, each member is separated by a
/// > comma and optional whitespace. For example, a field whose value is defined
/// > as a List of Tokens could look like:
/// >
/// > ```http
/// > Example-List: sugar, tea, rum
/// > ```
/// >
/// > Note that Lists can have their members split across multiple lines of the
/// > same header or trailer section, as per [Section
/// > 5.3](https://rfc-editor.org/rfc/rfc9110#section-5.3) of
/// > [[HTTP](https://www.rfc-editor.org/rfc/rfc9651#RFC9110)]; for
/// > example, the following are equivalent:
/// >
/// > ```http
/// > Example-List: sugar, tea, rum
/// > ```
/// >
/// > and
/// >
/// > ```http
/// > Example-List: sugar, tea
/// > Example-List: rum
/// > ```
/// >
/// > However, individual members of a List cannot be safely split between
/// > lines; see [Section
/// > 4.2](https://www.rfc-editor.org/rfc/rfc9651#text-parse) for details.
/// >
/// > Parsers MUST support Lists containing at least 1024 members. Field
/// > specifications can constrain the types and cardinality of individual List
/// > values as they require.
///
/// &mdash; [3.1. Lists | RFC 9651](https://www.rfc-editor.org/rfc/rfc9651#name-lists)
pub struct List();

/// > Parsers MUST support Lists containing at least 1024 members. Field
/// > specifications can constrain the types and cardinality of individual List
/// > values as they require.
///
/// &mdash; [3.1. Lists | RFC 9651](https://www.rfc-editor.org/rfc/rfc9651#name-lists)
pub const MAX_LIST_LEN: usize = 1024;

/// > Display Strings are similar to Strings, in that they consist of zero or
/// > more characters, but they allow Unicode scalar values (i.e., all Unicode
/// > code points except for surrogates), unlike Strings.
/// >
/// > Display Strings are intended for use in cases where a value is displayed
/// > to end users and therefore may need to carry non-ASCII content. It is NOT
/// > RECOMMENDED that they be used in situations where a String ([Section
/// > 3.3.3](https://www.rfc-editor.org/rfc/rfc9651#string)) or Token
/// > ([Section 3.3.4](https://www.rfc-editor.org/rfc/rfc9651#token)) would
/// > be adequate because Unicode has processing considerations (e.g.,
/// > normalization) and security considerations (e.g., homograph attacks) that
/// > make it more difficult to handle correctly.
/// >
/// > Note that Display Strings do not indicate the language used in the value;
/// > that can be done separately if necessary (e.g., with a parameter).
/// >
/// > In textual HTTP fields, Display Strings are represented in a manner
/// > similar to Strings, except that non-ASCII characters are percent-encoded;
/// > there is a leading "%" to distinguish them from Strings.
/// >
/// > For example:
/// >
/// > ```http
/// > Example-DisplayString: %"This is intended for display to %c3%bcsers."
/// > ```
/// >
/// > See [Section 6](https://www.rfc-editor.org/rfc/rfc9651#security) for
/// > additional security considerations when handling Display Strings.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct DisplayString(String);

impl Display for DisplayString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

// mod parsing;
