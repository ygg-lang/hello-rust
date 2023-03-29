use super::*;

impl YggdrasilNode for ValueNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        self.span.clone()
    }
}
impl YggdrasilNode for ObjectNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        self.span.clone()
    }
}
impl YggdrasilNode for ObjectPairNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        self.span.clone()
    }
}
impl YggdrasilNode for ArrayNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        self.span.clone()
    }
}
impl YggdrasilNode for StringNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        self.span.clone()
    }
}
impl YggdrasilNode for StringEscapedNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        self.span.clone()
    }
}
impl YggdrasilNode for NumberNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        self.span.clone()
    }
}
impl YggdrasilNode for BooleanNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        self.span.clone()
    }
}
impl YggdrasilNode for NullNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        self.span.clone()
    }
}
impl YggdrasilNode for IdentifierNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        self.span.clone()
    }
}
impl YggdrasilNode for WhiteSpaceNode {
    type Language = Json5Language;

    fn get_language(&self) -> Self::Language {
        Json5Language {}
    }

    fn get_range(&self) -> Range<usize> {
        self.span.clone()
    }
}
