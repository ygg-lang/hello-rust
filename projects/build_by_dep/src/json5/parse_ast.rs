use super::*;

impl YggdrasilNode for ValueNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unreachable!(),
        }
    }
}
impl YggdrasilNode for ObjectNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
}
impl YggdrasilNode for ObjectPairNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
}

impl YggdrasilNode for ObjectKeyNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unreachable!(),
        }
    }
}
impl YggdrasilNode for ArrayNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
}
impl YggdrasilNode for StringNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unreachable!(),
        }
    }
}
impl YggdrasilNode for StringRawNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
}
impl YggdrasilNode for StringTextNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unreachable!(),
        }
    }
}

impl YggdrasilNode for StringEscapeNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
}
impl YggdrasilNode for NumberNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
}

impl YggdrasilNode for IdentifierNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
}
impl YggdrasilNode for BooleanNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unreachable!(),
        }
    }
}
impl YggdrasilNode for NullNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
}

impl YggdrasilNode for WhiteSpaceNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
}

impl YggdrasilNode for CommentNode {
    type Rule = Json5Rule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            _ => unreachable!(),
        }
    }
}
