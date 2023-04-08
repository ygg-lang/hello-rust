use super::*;

#[automatically_derived]
impl YggdrasilNode for ValueNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<ArrayNode>(Cow::Borrowed("array")) {
            return Ok(Self::Array(s));
        }
        if let Ok(s) = pair.take_tagged_one::<BooleanNode>(Cow::Borrowed("boolean")) {
            return Ok(Self::Boolean(s));
        }
        if let Ok(s) = pair.take_tagged_one::<NullNode>(Cow::Borrowed("null")) {
            return Ok(Self::Null(s));
        }
        if let Ok(s) = pair.take_tagged_one::<NumberNode>(Cow::Borrowed("number")) {
            return Ok(Self::Number(s));
        }
        if let Ok(s) = pair.take_tagged_one::<ObjectNode>(Cow::Borrowed("object")) {
            return Ok(Self::Object(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringNode>(Cow::Borrowed("string")) {
            return Ok(Self::String(s));
        }
        Err(YggdrasilError::invalid_node(Json5Rule::Value, _span))
    }
}

#[automatically_derived]
impl YggdrasilNode for ObjectNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            object_pair: pair.take_tagged_items::<ObjectPairNode>(Cow::Borrowed("object_pair"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}

#[automatically_derived]
impl YggdrasilNode for ObjectPairNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            object_key: pair.take_tagged_one::<ObjectKeyNode>(Cow::Borrowed("object_key"))?,
            value: pair.take_tagged_one::<ValueNode>(Cow::Borrowed("value"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}

#[automatically_derived]
impl YggdrasilNode for ObjectKeyNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<IdentifierNode>(Cow::Borrowed("identifier")) {
            return Ok(Self::Identifier(s));
        }
        if let Ok(s) = pair.take_tagged_one::<IntegerNode>(Cow::Borrowed("integer")) {
            return Ok(Self::Integer(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringNode>(Cow::Borrowed("string")) {
            return Ok(Self::String(s));
        }
        Err(YggdrasilError::invalid_node(Json5Rule::ObjectKey, _span))
    }
}

#[automatically_derived]
impl YggdrasilNode for ArrayNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            value: pair.take_tagged_items::<ValueNode>(Cow::Borrowed("value"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}

#[automatically_derived]
impl YggdrasilNode for StringNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<StringRawNode>(Cow::Borrowed("string_0")) {
            return Ok(Self::String0(s));
        }
        if let Ok(s) = pair.take_tagged_one::<StringTextNode>(Cow::Borrowed("string_1")) {
            return Ok(Self::String1(s));
        }
        Err(YggdrasilError::invalid_node(Json5Rule::String, _span))
    }
}

#[automatically_derived]
impl YggdrasilNode for StringRawNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}

#[automatically_derived]
impl YggdrasilNode for StringTextNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<StringEscapeNode>(Cow::Borrowed("string_escape")) {
            return Ok(Self::StringEscape(s));
        }
        if let Some(_) = pair.find_first_tag("string_text_1") {
            return Ok(Self::StringText1);
        }
        Err(YggdrasilError::invalid_node(Json5Rule::StringText, _span))
    }
}

#[automatically_derived]
impl YggdrasilNode for StringEscapeNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}

#[automatically_derived]
impl YggdrasilNode for NumberNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}

#[automatically_derived]
impl YggdrasilNode for IntegerNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}

#[automatically_derived]
impl YggdrasilNode for IdentifierNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}

#[automatically_derived]
impl YggdrasilNode for BooleanNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unimplemented!(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Some(_) = pair.find_first_tag("boolean_0") {
            return Ok(Self::Boolean0);
        }
        if let Some(_) = pair.find_first_tag("boolean_1") {
            return Ok(Self::Boolean1);
        }
        Err(YggdrasilError::invalid_node(Json5Rule::Boolean, _span))
    }
}

#[automatically_derived]
impl YggdrasilNode for NullNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}

#[automatically_derived]
impl YggdrasilNode for WhiteSpaceNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
