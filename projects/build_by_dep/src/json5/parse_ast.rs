use super::*;

impl YggdrasilNode for ValueNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            _ => unreachable!(),
        }
    }
}
impl YggdrasilNode for ObjectNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl YggdrasilNode for ObjectPairNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            _ => unreachable!(),
        }
    }
}
impl YggdrasilNode for ArrayNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl YggdrasilNode for StringNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            _ => unreachable!(),
        }
    }
}

impl YggdrasilNode for StringRawNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}

impl YggdrasilNode for StringTextNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            _ => unreachable!(),
        }
    }
}
impl YggdrasilNode for NumberNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl YggdrasilNode for BooleanNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        match self {
            _ => unreachable!(),
        }
    }
}
impl YggdrasilNode for NullNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl YggdrasilNode for IdentifierNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl YggdrasilNode for WhiteSpaceNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
