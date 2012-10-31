/**
Constructs a list of css style rules from a token stream
*/

// TODO: fail according to the css spec instead of failing when things
// are not as expected

use values::*;
use util::{DataStream, DataStreamFactory};
use std::cell::Cell;
use netsurfcss::stylesheet::{CssStylesheetParams, CssStylesheetParamsVersion1, css_stylesheet_create};
use netsurfcss::types::CssLevel21;
use netsurfcss::CssResult;
use wapcaplet::LwcString;

// This takes a DataStreamFactory instead of a DataStream because
// servo's DataStream contains a comm::Port, which is not sendable,
// so DataStream is an @fn which can't be sent to the lexer task.
// So the DataStreamFactory gives the caller an opportunity to create
// the data stream from inside the lexer task.
pub fn parse_stylesheet(url: Url, input_factory: DataStreamFactory) -> Stylesheet {
    let params: CssStylesheetParams = CssStylesheetParams {
        params_version: CssStylesheetParamsVersion1,
        level: CssLevel21,
        charset: ~"UTF-8",
        url: url.to_str(),
        title: ~"FIXME-css-title",
        allow_quirks: false,
        inline_style: false,
        resolve: Some(resolve_url),
        import: None,
        color: None,
        font: None,
    };
    let sheet = css_stylesheet_create(move params);

    let input = input_factory();
    loop {
        match input() {
            Some(move data) => {
                sheet.append_data(data);
            }
            None => break
        }
    }
    sheet.data_done();
    return move sheet;
}

fn resolve_url(_base: &str, _rel: &LwcString) -> CssResult<LwcString> {
    fail ~"resolving url";
}